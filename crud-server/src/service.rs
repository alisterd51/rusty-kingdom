use crate::{
    DbPool,
    pb::crud::v1::{
        CollectFortressResourceRequest, CollectFortressResourceResponse, CreateBuildingRequest,
        CreateBuildingResponse, CreateFortressRequest, CreateFortressResponse,
        DeleteBuildingRequest, DeleteBuildingResponse, DeleteFortressRequest,
        DeleteFortressResponse, GetBuildingRequest, GetBuildingResponse, GetFortressRequest,
        GetFortressResponse, ListBuildingsByFortressRequest, ListBuildingsByFortressResponse,
        ListBuildingsRequest, ListBuildingsResponse, ListFortressesRequest, ListFortressesResponse,
        ResourceKind, UpdateBuildingRequest, UpdateBuildingResponse, UpdateFortressRequest,
        UpdateFortressResponse, UpgradeBuildingAtomicRequest, UpgradeBuildingAtomicResponse,
        building_service_server::BuildingService, fortress_service_server::FortressService,
    },
};
use diesel::{
    prelude::*,
    sql_types::{Integer, Text},
};
use rusty::{
    models::{Building, Fortress, NewBuilding, NewFortress, UpdateBuilding, UpdateFortress},
    schema::{buildings, fortresses},
};
use std::sync::Arc;
use tonic::{Request, Response, Status};

impl From<Building> for crate::pb::common::v1::Building {
    fn from(value: Building) -> Self {
        Self {
            id: value.id,
            name: value.name,
            level: value.level,
            fortress_id: value.fortress_id,
        }
    }
}

impl From<crate::pb::common::v1::NewBuilding> for NewBuilding {
    fn from(value: crate::pb::common::v1::NewBuilding) -> Self {
        Self {
            name: value.name,
            level: value.level,
            fortress_id: value.fortress_id,
        }
    }
}

impl From<crate::pb::common::v1::UpdateBuilding> for UpdateBuilding {
    fn from(value: crate::pb::common::v1::UpdateBuilding) -> Self {
        Self {
            name: value.name,
            level: value.level,
            fortress_id: value.fortress_id,
        }
    }
}

impl From<Fortress> for crate::pb::common::v1::Fortress {
    fn from(value: Fortress) -> Self {
        Self {
            id: value.id,
            gold: value.gold,
            food: value.food,
            wood: value.wood,
            energy: value.energy,
        }
    }
}

impl From<crate::pb::common::v1::NewFortress> for NewFortress {
    fn from(value: crate::pb::common::v1::NewFortress) -> Self {
        Self {
            gold: value.gold,
            food: value.food,
            wood: value.wood,
            energy: value.energy,
        }
    }
}

impl From<crate::pb::common::v1::UpdateFortress> for UpdateFortress {
    fn from(value: crate::pb::common::v1::UpdateFortress) -> Self {
        Self {
            gold: value.gold,
            food: value.food,
            wood: value.wood,
            energy: value.energy,
        }
    }
}

#[derive(diesel::QueryableByName)]
struct FortressRow {
    #[diesel(sql_type = Integer)]
    id: i32,
    #[diesel(sql_type = Integer)]
    gold: i32,
    #[diesel(sql_type = Integer)]
    food: i32,
    #[diesel(sql_type = Integer)]
    wood: i32,
    #[diesel(sql_type = Integer)]
    energy: i32,
}

impl From<FortressRow> for crate::pb::common::v1::Fortress {
    fn from(v: FortressRow) -> Self {
        Self {
            id: v.id,
            gold: v.gold,
            food: v.food,
            wood: v.wood,
            energy: v.energy,
        }
    }
}

#[derive(Debug)]
enum UpgradeBuildingAtomicError {
    Diesel(diesel::result::Error),
    BuildingNotFound,
    FortressNotFound,
    MaxLevel,
    InsufficientResources,
    ConcurrentUpdate,
}

impl From<diesel::result::Error> for UpgradeBuildingAtomicError {
    fn from(value: diesel::result::Error) -> Self {
        Self::Diesel(value)
    }
}

pub struct MyBuildingService {
    pool: Arc<DbPool>,
}

impl MyBuildingService {
    #[must_use]
    pub const fn new(pool: Arc<DbPool>) -> Self {
        Self { pool }
    }
}

#[tonic::async_trait]
impl BuildingService for MyBuildingService {
    async fn create_building(
        &self,
        request: Request<CreateBuildingRequest>,
    ) -> Result<Response<CreateBuildingResponse>, Status> {
        let new_building: NewBuilding = request
            .into_inner()
            .building
            .ok_or_else(|| Status::not_found("missing building field"))?
            .into();
        let mut conn = self
            .pool
            .get()
            .map_err(|e| Status::internal(format!("{e}")))?;
        let building = diesel::insert_into(buildings::table)
            .values(new_building)
            .returning(Building::as_returning())
            .get_result(&mut conn)
            .map_err(|e| Status::internal(format!("{e}")))?;
        let building = CreateBuildingResponse {
            building: Some(building.into()),
        };

        Ok(Response::new(building))
    }

    async fn get_building(
        &self,
        request: Request<GetBuildingRequest>,
    ) -> Result<Response<GetBuildingResponse>, Status> {
        let building_id = request.into_inner().id;
        let mut conn = self
            .pool
            .get()
            .map_err(|e| Status::internal(format!("{e}")))?;
        let building: Building = buildings::table
            .filter(buildings::id.eq(building_id))
            .get_result(&mut conn)
            .map_err(|e| Status::not_found(format!("{e}")))?;
        let building = GetBuildingResponse {
            building: Some(building.into()),
        };

        Ok(Response::new(building))
    }

    async fn update_building(
        &self,
        request: Request<UpdateBuildingRequest>,
    ) -> Result<Response<UpdateBuildingResponse>, Status> {
        let update_building = request
            .into_inner()
            .building
            .ok_or_else(|| Status::not_found("missing building field"))?;
        let building_id = update_building.id;
        let update_building: UpdateBuilding = update_building.into();
        let mut conn = self
            .pool
            .get()
            .map_err(|e| Status::internal(format!("{e}")))?;
        let building = diesel::update(buildings::table)
            .filter(buildings::id.eq(building_id))
            .set(update_building)
            .returning(Building::as_returning())
            .get_result(&mut conn)
            .map_err(|e| Status::internal(format!("{e}")))?;
        let building = UpdateBuildingResponse {
            building: Some(building.into()),
        };

        Ok(Response::new(building))
    }

    async fn delete_building(
        &self,
        request: Request<DeleteBuildingRequest>,
    ) -> Result<Response<DeleteBuildingResponse>, Status> {
        let building_id = request.into_inner().id;
        let mut conn = self
            .pool
            .get()
            .map_err(|e| Status::internal(format!("{e}")))?;
        let building_delete_result = diesel::delete(buildings::table)
            .filter(buildings::id.eq(building_id))
            .execute(&mut conn)
            .map_err(|e| Status::internal(format!("{e}")))?;
        let success = building_delete_result != 0;
        let success = DeleteBuildingResponse { success };

        Ok(Response::new(success))
    }

    async fn list_buildings(
        &self,
        _request: Request<ListBuildingsRequest>,
    ) -> Result<Response<ListBuildingsResponse>, Status> {
        let mut conn = self
            .pool
            .get()
            .map_err(|e| Status::internal(format!("{e}")))?;
        let buildings: Vec<Building> = buildings::table
            .get_results(&mut conn)
            .map_err(|e| Status::internal(format!("{e}")))?;
        let buildings = ListBuildingsResponse {
            buildings: buildings.into_iter().map(Into::into).collect(),
        };

        Ok(Response::new(buildings))
    }

    async fn list_buildings_by_fortress(
        &self,
        request: Request<ListBuildingsByFortressRequest>,
    ) -> Result<Response<ListBuildingsByFortressResponse>, Status> {
        let mut conn = self
            .pool
            .get()
            .map_err(|e| Status::internal(format!("{e}")))?;
        let fortress_id = request.into_inner().fortress_id;
        let buildings: Vec<Building> = buildings::table
            .filter(buildings::fortress_id.eq(fortress_id))
            .get_results(&mut conn)
            .map_err(|e| Status::internal(format!("{e}")))?;
        let buildings = ListBuildingsByFortressResponse {
            buildings: buildings.into_iter().map(Into::into).collect(),
        };

        Ok(Response::new(buildings))
    }

    async fn upgrade_building_atomic(
        &self,
        request: Request<UpgradeBuildingAtomicRequest>,
    ) -> Result<Response<UpgradeBuildingAtomicResponse>, Status> {
        let req = request.into_inner();
        let building_id = req.building_id;
        let max_building_level = req.max_building_level;
        if max_building_level <= 0 {
            return Err(Status::invalid_argument("max_building_level must be > 0"));
        }
        let costs = req
            .costs
            .ok_or_else(|| Status::invalid_argument("missing costs field"))?;
        if costs.gold < 0 || costs.food < 0 || costs.wood < 0 || costs.energy < 0 {
            return Err(Status::invalid_argument("costs must be non-negative"));
        }
        let expected_level = req.expected_building_level;
        let mut conn = self
            .pool
            .get()
            .map_err(|e| Status::internal(format!("{e}")))?;
        let result: Result<(Fortress, Building), UpgradeBuildingAtomicError> =
            conn.transaction(|conn| {
                let building = buildings::table
                    .filter(buildings::id.eq(building_id))
                    .select(Building::as_select())
                    .first(conn)
                    .optional()?
                    .ok_or(UpgradeBuildingAtomicError::BuildingNotFound)?;
                if let Some(expected) = expected_level
                    && expected != building.level
                {
                    return Err(UpgradeBuildingAtomicError::ConcurrentUpdate);
                }
                if building.level >= max_building_level {
                    return Err(UpgradeBuildingAtomicError::MaxLevel);
                }
                let fortress_id = building.fortress_id;
                let fortress = diesel::update(fortresses::table)
                    .filter(fortresses::id.eq(fortress_id))
                    .filter(fortresses::gold.ge(costs.gold))
                    .filter(fortresses::food.ge(costs.food))
                    .filter(fortresses::wood.ge(costs.wood))
                    .filter(fortresses::energy.ge(costs.energy))
                    .set((
                        fortresses::gold.eq(fortresses::gold - costs.gold),
                        fortresses::food.eq(fortresses::food - costs.food),
                        fortresses::wood.eq(fortresses::wood - costs.wood),
                        fortresses::energy.eq(fortresses::energy - costs.energy),
                    ))
                    .returning(Fortress::as_returning())
                    .get_result(conn)
                    .optional()?;
                let Some(fortress) = fortress else {
                    let exists = fortresses::table
                        .filter(fortresses::id.eq(fortress_id))
                        .select(Fortress::as_select())
                        .first(conn)
                        .optional()?;
                    if exists.is_some() {
                        return Err(UpgradeBuildingAtomicError::InsufficientResources);
                    }
                    return Err(UpgradeBuildingAtomicError::FortressNotFound);
                };
                let upgraded_building = diesel::update(buildings::table)
                    .filter(buildings::id.eq(building_id))
                    .filter(buildings::level.eq(building.level))
                    .set(buildings::level.eq(buildings::level + 1))
                    .returning(Building::as_returning())
                    .get_result(conn)
                    .optional()?
                    .ok_or(UpgradeBuildingAtomicError::ConcurrentUpdate)?;

                Ok((fortress, upgraded_building))
            });

        match result {
            Ok((fortress, building)) => Ok(Response::new(UpgradeBuildingAtomicResponse {
                fortress: Some(fortress.into()),
                building: Some(building.into()),
            })),
            Err(UpgradeBuildingAtomicError::BuildingNotFound) => {
                Err(Status::not_found("building not found"))
            }
            Err(UpgradeBuildingAtomicError::FortressNotFound) => {
                Err(Status::not_found("fortress not found"))
            }
            Err(UpgradeBuildingAtomicError::MaxLevel) => {
                Err(Status::failed_precondition("building already at max level"))
            }
            Err(UpgradeBuildingAtomicError::InsufficientResources) => {
                Err(Status::failed_precondition("insufficient resources"))
            }
            Err(UpgradeBuildingAtomicError::ConcurrentUpdate) => {
                Err(Status::aborted("concurrent update; retry"))
            }
            Err(UpgradeBuildingAtomicError::Diesel(_e)) => Err(Status::internal("db error")),
        }
    }
}

pub struct MyFortressService {
    pool: Arc<DbPool>,
}

impl MyFortressService {
    #[must_use]
    pub const fn new(pool: Arc<DbPool>) -> Self {
        Self { pool }
    }
}

#[tonic::async_trait]
impl FortressService for MyFortressService {
    async fn create_fortress(
        &self,
        request: Request<CreateFortressRequest>,
    ) -> Result<Response<CreateFortressResponse>, Status> {
        let new_fortress: NewFortress = request
            .into_inner()
            .fortress
            .ok_or_else(|| Status::not_found("missing fortress field"))?
            .into();
        let mut conn = self
            .pool
            .get()
            .map_err(|e| Status::internal(format!("{e}")))?;
        let fortress = diesel::insert_into(fortresses::table)
            .values(new_fortress)
            .returning(Fortress::as_returning())
            .get_result(&mut conn)
            .map_err(|e| Status::internal(format!("{e}")))?;
        let fortress = CreateFortressResponse {
            fortress: Some(fortress.into()),
        };

        Ok(Response::new(fortress))
    }

    async fn get_fortress(
        &self,
        request: Request<GetFortressRequest>,
    ) -> Result<Response<GetFortressResponse>, Status> {
        let fortress_id = request.into_inner().id;
        let mut conn = self
            .pool
            .get()
            .map_err(|e| Status::internal(format!("{e}")))?;
        let fortress: Fortress = fortresses::table
            .filter(fortresses::id.eq(fortress_id))
            .get_result(&mut conn)
            .map_err(|e| Status::not_found(format!("{e}")))?;
        let fortress = GetFortressResponse {
            fortress: Some(fortress.into()),
        };

        Ok(Response::new(fortress))
    }

    async fn update_fortress(
        &self,
        request: Request<UpdateFortressRequest>,
    ) -> Result<Response<UpdateFortressResponse>, Status> {
        let update_fortress = request
            .into_inner()
            .fortress
            .ok_or_else(|| Status::not_found("missing fortress field"))?;
        let fortress_id = update_fortress.id;
        let update_fortress: UpdateFortress = update_fortress.into();
        let mut conn = self
            .pool
            .get()
            .map_err(|e| Status::internal(format!("{e}")))?;
        let fortress = diesel::update(fortresses::table)
            .filter(fortresses::id.eq(fortress_id))
            .set(update_fortress)
            .returning(Fortress::as_returning())
            .get_result(&mut conn)
            .map_err(|e| Status::internal(format!("{e}")))?;
        let fortress = UpdateFortressResponse {
            fortress: Some(fortress.into()),
        };

        Ok(Response::new(fortress))
    }

    async fn delete_fortress(
        &self,
        request: Request<DeleteFortressRequest>,
    ) -> Result<Response<DeleteFortressResponse>, Status> {
        let fortress_id = request.into_inner().id;
        let mut conn = self
            .pool
            .get()
            .map_err(|e| Status::internal(format!("{e}")))?;
        let _building_delete_result = diesel::delete(buildings::table)
            .filter(buildings::fortress_id.eq(fortress_id))
            .execute(&mut conn)
            .map_err(|e| Status::internal(format!("{e}")))?;
        let fortress_delete_result = diesel::delete(fortresses::table)
            .filter(fortresses::id.eq(fortress_id))
            .execute(&mut conn)
            .map_err(|e| Status::internal(format!("{e}")))?;
        let success = fortress_delete_result != 0;
        let success = DeleteFortressResponse { success };

        Ok(Response::new(success))
    }

    async fn list_fortresses(
        &self,
        _request: Request<ListFortressesRequest>,
    ) -> Result<Response<ListFortressesResponse>, Status> {
        let mut conn = self
            .pool
            .get()
            .map_err(|e| Status::internal(format!("{e}")))?;
        let fortresses: Vec<Fortress> = fortresses::table
            .get_results(&mut conn)
            .map_err(|e| Status::internal(format!("{e}")))?;
        let fortresses = ListFortressesResponse {
            fortresses: fortresses.into_iter().map(Into::into).collect(),
        };

        Ok(Response::new(fortresses))
    }

    async fn collect_fortress_resource(
        &self,
        request: Request<CollectFortressResourceRequest>,
    ) -> Result<Response<CollectFortressResourceResponse>, Status> {
        let req = request.into_inner();
        let fortress_id = req.id;
        let bonus_building_name = req.bonus_building_name;
        let base = req.base.unwrap_or(1);
        let resource = ResourceKind::try_from(req.resource)
            .map_err(|_| Status::invalid_argument("invalid resource kind"))?;
        let sql = match resource {
            ResourceKind::Gold => {
                r"
                UPDATE fortresses
                SET gold = gold + $2
                    + COALESCE((SELECT SUM(level)::int
                                FROM buildings
                                WHERE fortress_id = $1 AND name = $3), 0)
                WHERE id = $1
                RETURNING id, gold, food, wood, energy
            "
            }
            ResourceKind::Food => {
                r"
                UPDATE fortresses
                SET food = food + $2
                    + COALESCE((SELECT SUM(level)::int
                                FROM buildings
                                WHERE fortress_id = $1 AND name = $3), 0)
                WHERE id = $1
                RETURNING id, gold, food, wood, energy
            "
            }
            ResourceKind::Wood => {
                r"
                UPDATE fortresses
                SET wood = wood + $2
                    + COALESCE((SELECT SUM(level)::int
                                FROM buildings
                                WHERE fortress_id = $1 AND name = $3), 0)
                WHERE id = $1
                RETURNING id, gold, food, wood, energy
            "
            }
            ResourceKind::Energy => {
                r"
                UPDATE fortresses
                SET energy = energy + $2
                    + COALESCE((SELECT SUM(level)::int
                                FROM buildings
                                WHERE fortress_id = $1 AND name = $3), 0)
                WHERE id = $1
                RETURNING id, gold, food, wood, energy
            "
            }
            ResourceKind::Unspecified => {
                return Err(Status::invalid_argument("resource kind is required"));
            }
        };
        let mut conn = self
            .pool
            .get()
            .map_err(|e| Status::internal(format!("{e}")))?;
        let fortress_row: FortressRow = diesel::sql_query(sql)
            .bind::<Integer, _>(fortress_id)
            .bind::<Integer, _>(base)
            .bind::<Text, _>(bonus_building_name)
            .get_result(&mut conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => Status::not_found("fortress not found"),
                _ => Status::internal("db error"),
            })?;

        Ok(Response::new(CollectFortressResourceResponse {
            fortress: Some(fortress_row.into()),
        }))
    }
}
