use crate::{
    DbPool,
    crud::{
        CreateBuildingRequest, CreateBuildingResponse, CreateFortressRequest,
        CreateFortressResponse, DeleteBuildingRequest, DeleteBuildingResponse,
        DeleteFortressRequest, DeleteFortressResponse, GetBuildingRequest, GetBuildingResponse,
        GetFortressRequest, GetFortressResponse, ListBuildingsByFortressRequest,
        ListBuildingsByFortressResponse, ListBuildingsRequest, ListBuildingsResponse,
        ListFortressesRequest, ListFortressesResponse, UpdateBuildingRequest,
        UpdateBuildingResponse, UpdateFortressRequest, UpdateFortressResponse,
        building_service_server::BuildingService, fortress_service_server::FortressService,
    },
};
use diesel::prelude::*;
use rusty::{
    models::{NewBuilding, NewFortress, UpdateBuilding, UpdateFortress},
    schema::{buildings, fortresses},
};
use std::sync::Arc;
use tonic::{Request, Response, Status};

impl From<rusty::models::Building> for crate::common::Building {
    fn from(value: rusty::models::Building) -> Self {
        Self {
            id: value.id,
            name: value.name,
            level: value.level,
            fortress_id: value.fortress_id,
        }
    }
}

impl From<crate::common::NewBuilding> for rusty::models::NewBuilding {
    fn from(value: crate::common::NewBuilding) -> Self {
        Self {
            name: value.name,
            level: value.level,
            fortress_id: value.fortress_id,
        }
    }
}

impl From<crate::common::UpdateBuilding> for rusty::models::UpdateBuilding {
    fn from(value: crate::common::UpdateBuilding) -> Self {
        Self {
            name: value.name,
            level: value.level,
            fortress_id: value.fortress_id,
        }
    }
}

impl From<rusty::models::Fortress> for crate::common::Fortress {
    fn from(value: rusty::models::Fortress) -> Self {
        Self {
            id: value.id,
            gold: value.gold,
            food: value.food,
            wood: value.wood,
            energy: value.energy,
        }
    }
}

impl From<crate::common::NewFortress> for rusty::models::NewFortress {
    fn from(value: crate::common::NewFortress) -> Self {
        Self {
            gold: value.gold,
            food: value.food,
            wood: value.wood,
            energy: value.energy,
        }
    }
}

impl From<crate::common::UpdateFortress> for rusty::models::UpdateFortress {
    fn from(value: crate::common::UpdateFortress) -> Self {
        Self {
            gold: value.gold,
            food: value.food,
            wood: value.wood,
            energy: value.energy,
        }
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
            .returning(rusty::models::Building::as_returning())
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
        let building: rusty::models::Building = buildings::table
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
            .returning(rusty::models::Building::as_returning())
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
        let buildings: Vec<rusty::models::Building> = buildings::table
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
        let buildings: Vec<rusty::models::Building> = buildings::table
            .filter(buildings::fortress_id.eq(fortress_id))
            .get_results(&mut conn)
            .map_err(|e| Status::internal(format!("{e}")))?;
        let buildings = ListBuildingsByFortressResponse {
            buildings: buildings.into_iter().map(Into::into).collect(),
        };

        Ok(Response::new(buildings))
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
            .returning(rusty::models::Fortress::as_returning())
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
        let fortress: rusty::models::Fortress = fortresses::table
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
            .returning(rusty::models::Fortress::as_returning())
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
        let fortresses: Vec<rusty::models::Fortress> = fortresses::table
            .get_results(&mut conn)
            .map_err(|e| Status::internal(format!("{e}")))?;
        let fortresses = ListFortressesResponse {
            fortresses: fortresses.into_iter().map(Into::into).collect(),
        };

        Ok(Response::new(fortresses))
    }
}
