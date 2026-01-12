use crate::pb::{
    common::v1::{
        Building, Costs, Fortress, NewBuilding, NewFortress, UpdateBuilding, UpdateFortress,
    },
    crud::v1::{
        UpdateBuildingRequest, UpdateFortressRequest,
        building_service_client::BuildingServiceClient,
        fortress_service_client::FortressServiceClient,
    },
    game::v1::{
        CollectFortressEnergyRequest, CollectFortressEnergyResponse, CollectFortressFoodRequest,
        CollectFortressFoodResponse, CollectFortressGoldRequest, CollectFortressGoldResponse,
        CollectFortressWoodRequest, CollectFortressWoodResponse, CreateFortressRequest,
        CreateFortressResponse, DeleteFortressRequest, DeleteFortressResponse, GetBuildingRequest,
        GetBuildingResponse, GetFortressEnergyRequest, GetFortressEnergyResponse,
        GetFortressFoodRequest, GetFortressFoodResponse, GetFortressGoldRequest,
        GetFortressGoldResponse, GetFortressRequest, GetFortressResponse, GetFortressWoodRequest,
        GetFortressWoodResponse, GetImproveBuildingCostsRequest, GetImproveBuildingCostsResponse,
        ImproveBuildingRequest, ImproveBuildingResponse, ListBuildingsByFortressRequest,
        ListBuildingsByFortressResponse, ListBuildingsRequest, ListBuildingsResponse,
        ListFortressesRequest, ListFortressesResponse, building_service_server::BuildingService,
        fortress_service_server::FortressService,
    },
};
use tonic::{Request, Response, Status};

const MAX_BUILDING_LEVEL: i32 = 20;

fn upgrade_building(
    fortress: &Fortress,
    building: &Building,
    costs: &Costs,
    max_building_level: i32,
) -> Result<(UpdateFortress, UpdateBuilding), String> {
    if fortress.id != building.fortress_id {
        return Err(String::from("fortress id mismatch"));
    }
    if building.level >= max_building_level {
        return Err(String::from("building level is too high"));
    }
    if fortress.gold < costs.gold
        || fortress.food < costs.food
        || fortress.wood < costs.wood
        || fortress.energy < costs.energy
    {
        let error = format!("insufficient resources: {fortress:?} {costs:?}");
        return Err(error);
    }
    let update_fortress = UpdateFortress {
        id: fortress.id,
        gold: Some(fortress.gold - costs.gold),
        food: Some(fortress.food - costs.food),
        wood: Some(fortress.wood - costs.wood),
        energy: Some(fortress.energy - costs.energy),
    };
    let update_building = UpdateBuilding {
        id: building.id,
        name: None,
        level: Some(building.level + 1),
        fortress_id: None,
    };
    Ok((update_fortress, update_building))
}

fn upgrade_cost(level: i32, base: i32, factor: f64) -> f64 {
    let level = if level < 1 { 1 } else { level };
    f64::from(base) * factor.powf(f64::from(level - 1))
}

fn optimize_factor(level_max: i32, base: i32, cost_max: i32) -> f64 {
    (f64::from(cost_max) / f64::from(base)).powf(1.0 / (f64::from(level_max) - 1.0))
}

#[allow(clippy::cast_possible_truncation)]
fn get_costs(level: i32, level_max: i32) -> Costs {
    let factor = optimize_factor(level_max, 10, i32::MAX);
    let cost = upgrade_cost(level, 10, factor) as i32;

    Costs {
        gold: cost / 2,
        food: cost,
        wood: cost / 5,
        energy: cost / 10,
    }
}

fn get_gold_bonus(buildings: Vec<Building>) -> i32 {
    let mut gold_bonus = 0;

    for building in buildings {
        if building.name == "bank" {
            gold_bonus += building.level;
        }
    }
    gold_bonus
}

fn get_food_bonus(buildings: Vec<Building>) -> i32 {
    let mut food_bonus = 0;

    for building in buildings {
        if building.name == "farm" {
            food_bonus += building.level;
        }
    }
    food_bonus
}

fn get_wood_bonus(buildings: Vec<Building>) -> i32 {
    let mut wood_bonus = 0;

    for building in buildings {
        if building.name == "sawmill" {
            wood_bonus += building.level;
        }
    }
    wood_bonus
}

fn get_energy_bonus(buildings: Vec<Building>) -> i32 {
    let mut energy_bonus = 0;

    for building in buildings {
        if building.name == "sanctuary" {
            energy_bonus += building.level;
        }
    }
    energy_bonus
}

pub struct MyBuildingService {
    crud_building_client: BuildingServiceClient<tonic::transport::Channel>,
    crud_fortress_client: FortressServiceClient<tonic::transport::Channel>,
}

impl MyBuildingService {
    pub const fn new(
        crud_building_client: BuildingServiceClient<tonic::transport::Channel>,
        crud_fortress_client: FortressServiceClient<tonic::transport::Channel>,
    ) -> Self {
        Self {
            crud_building_client,
            crud_fortress_client,
        }
    }
}

#[tonic::async_trait]
impl BuildingService for MyBuildingService {
    async fn get_building(
        &self,
        request: Request<GetBuildingRequest>,
    ) -> Result<Response<GetBuildingResponse>, Status> {
        let crud_request = Request::new(crate::pb::crud::v1::GetBuildingRequest {
            id: request.into_inner().id,
        });
        let building = self
            .crud_building_client
            .clone()
            .get_building(crud_request)
            .await?;
        let buildings = GetBuildingResponse {
            building: building.into_inner().building,
        };
        Ok(Response::new(buildings))
    }

    async fn list_buildings(
        &self,
        _request: Request<ListBuildingsRequest>,
    ) -> Result<Response<ListBuildingsResponse>, Status> {
        let crud_request = Request::new(crate::pb::crud::v1::ListBuildingsRequest {});
        let buildings = self
            .crud_building_client
            .clone()
            .list_buildings(crud_request)
            .await?;
        let buildings = ListBuildingsResponse {
            buildings: buildings.into_inner().buildings,
        };
        Ok(Response::new(buildings))
    }

    async fn list_buildings_by_fortress(
        &self,
        request: Request<ListBuildingsByFortressRequest>,
    ) -> Result<Response<ListBuildingsByFortressResponse>, Status> {
        let crud_request = Request::new(crate::pb::crud::v1::ListBuildingsByFortressRequest {
            fortress_id: request.into_inner().fortress_id,
        });
        let buildings = self
            .crud_building_client
            .clone()
            .list_buildings_by_fortress(crud_request)
            .await?;
        let buildings = ListBuildingsByFortressResponse {
            buildings: buildings.into_inner().buildings,
        };
        Ok(Response::new(buildings))
    }

    async fn improve_building(
        &self,
        request: Request<ImproveBuildingRequest>,
    ) -> Result<Response<ImproveBuildingResponse>, Status> {
        let building_id = request.into_inner().id;
        let crud_request =
            Request::new(crate::pb::crud::v1::GetBuildingRequest { id: building_id });
        let building = self
            .crud_building_client
            .clone()
            .get_building(crud_request)
            .await?
            .into_inner()
            .building
            .ok_or_else(|| Status::not_found("building not found"))?;
        let fortress_id = building.fortress_id;
        let crud_request =
            Request::new(crate::pb::crud::v1::GetFortressRequest { id: fortress_id });
        let fortress = self
            .crud_fortress_client
            .clone()
            .get_fortress(crud_request)
            .await?
            .into_inner()
            .fortress
            .ok_or_else(|| Status::not_found("fortress not found"))?;
        let costs = get_costs(building.level, MAX_BUILDING_LEVEL);
        let (update_fortress, update_building) =
            upgrade_building(&fortress, &building, &costs, MAX_BUILDING_LEVEL)
                .map_err(Status::internal)?;
        let crud_update_request = UpdateFortressRequest {
            fortress: Some(update_fortress),
        };
        let fortress = self
            .crud_fortress_client
            .clone()
            .update_fortress(crud_update_request)
            .await?
            .into_inner()
            .fortress;

        let crud_update_request = UpdateBuildingRequest {
            building: Some(update_building),
        };
        let building = self
            .crud_building_client
            .clone()
            .update_building(crud_update_request)
            .await?
            .into_inner()
            .building;

        let message = ImproveBuildingResponse { fortress, building };

        Ok(Response::new(message))
    }

    async fn get_improve_building_costs(
        &self,
        request: Request<GetImproveBuildingCostsRequest>,
    ) -> Result<Response<GetImproveBuildingCostsResponse>, Status> {
        let crud_request = Request::new(crate::pb::crud::v1::GetBuildingRequest {
            id: request.into_inner().id,
        });
        let building_level = self
            .crud_building_client
            .clone()
            .get_building(crud_request)
            .await?
            .into_inner()
            .building
            .ok_or_else(|| Status::not_found("building not found"))?
            .level;
        let costs = get_costs(building_level, MAX_BUILDING_LEVEL);
        let message = GetImproveBuildingCostsResponse { costs: Some(costs) };
        Ok(Response::new(message))
    }
}

pub struct MyFortressService {
    crud_building_client: BuildingServiceClient<tonic::transport::Channel>,
    crud_fortress_client: FortressServiceClient<tonic::transport::Channel>,
}

impl MyFortressService {
    pub const fn new(
        crud_building_client: BuildingServiceClient<tonic::transport::Channel>,
        crud_fortress_client: FortressServiceClient<tonic::transport::Channel>,
    ) -> Self {
        Self {
            crud_building_client,
            crud_fortress_client,
        }
    }
}

#[tonic::async_trait]
impl FortressService for MyFortressService {
    async fn create_fortress(
        &self,
        _request: Request<CreateFortressRequest>,
    ) -> Result<Response<CreateFortressResponse>, Status> {
        let create_fortress_request = crate::pb::crud::v1::CreateFortressRequest {
            fortress: Some(NewFortress {
                gold: 0,
                food: 0,
                wood: 0,
                energy: 0,
            }),
        };
        let fortress = self
            .crud_fortress_client
            .clone()
            .create_fortress(create_fortress_request)
            .await?
            .into_inner()
            .fortress
            .ok_or_else(|| Status::not_found("fortress not found"))?;
        let new_buildings = vec![
            NewBuilding {
                name: "bank".to_string(),
                level: 0,
                fortress_id: fortress.id,
            },
            NewBuilding {
                name: "farm".to_string(),
                level: 0,
                fortress_id: fortress.id,
            },
            NewBuilding {
                name: "sawmill".to_string(),
                level: 0,
                fortress_id: fortress.id,
            },
            NewBuilding {
                name: "sanctuary".to_string(),
                level: 0,
                fortress_id: fortress.id,
            },
        ];
        let mut buildings = vec![];
        for new_building in new_buildings {
            let create_building_request = crate::pb::crud::v1::CreateBuildingRequest {
                building: Some(new_building),
            };
            let building = self
                .crud_building_client
                .clone()
                .create_building(create_building_request)
                .await?
                .into_inner()
                .building
                .ok_or_else(|| Status::not_found("building not found"))?;
            buildings.push(building);
        }
        let message = CreateFortressResponse {
            fortress: Some(fortress),
            buildings,
        };

        Ok(Response::new(message))
    }

    async fn get_fortress(
        &self,
        request: Request<GetFortressRequest>,
    ) -> Result<Response<GetFortressResponse>, Status> {
        let get_fortress_request = crate::pb::crud::v1::GetFortressRequest {
            id: request.into_inner().id,
        };
        let fortress = self
            .crud_fortress_client
            .clone()
            .get_fortress(get_fortress_request)
            .await?
            .into_inner()
            .fortress;

        let message = GetFortressResponse { fortress };

        Ok(Response::new(message))
    }

    async fn delete_fortress(
        &self,
        request: Request<DeleteFortressRequest>,
    ) -> Result<Response<DeleteFortressResponse>, Status> {
        let delete_fortress_request = crate::pb::crud::v1::DeleteFortressRequest {
            id: request.into_inner().id,
        };
        let success = self
            .crud_fortress_client
            .clone()
            .delete_fortress(delete_fortress_request)
            .await?
            .into_inner()
            .success;

        let message = DeleteFortressResponse { success };

        Ok(Response::new(message))
    }

    async fn list_fortresses(
        &self,
        _request: Request<ListFortressesRequest>,
    ) -> Result<Response<ListFortressesResponse>, Status> {
        let list_fortresses_request = crate::pb::crud::v1::ListFortressesRequest {};

        let fortresses = self
            .crud_fortress_client
            .clone()
            .list_fortresses(list_fortresses_request)
            .await?
            .into_inner()
            .fortresses;

        let message = ListFortressesResponse { fortresses };

        Ok(Response::new(message))
    }

    async fn get_fortress_gold(
        &self,
        request: Request<GetFortressGoldRequest>,
    ) -> Result<Response<GetFortressGoldResponse>, Status> {
        let get_fortress_request = crate::pb::crud::v1::GetFortressRequest {
            id: request.into_inner().id,
        };
        let gold = self
            .crud_fortress_client
            .clone()
            .get_fortress(get_fortress_request)
            .await?
            .into_inner()
            .fortress
            .ok_or_else(|| Status::not_found("fortress not found"))?
            .gold;

        let message = GetFortressGoldResponse { gold };

        Ok(Response::new(message))
    }

    async fn collect_fortress_gold(
        &self,
        request: Request<CollectFortressGoldRequest>,
    ) -> Result<Response<CollectFortressGoldResponse>, Status> {
        let fortress_id = request.into_inner().id;
        let get_fortress_request = crate::pb::crud::v1::GetFortressRequest { id: fortress_id };
        let fortress = self
            .crud_fortress_client
            .clone()
            .get_fortress(get_fortress_request)
            .await?
            .into_inner()
            .fortress
            .ok_or_else(|| Status::not_found("fortress not found"))?;
        let list_buildings_by_fortress_request =
            crate::pb::crud::v1::ListBuildingsByFortressRequest { fortress_id };
        let buildings = self
            .crud_building_client
            .clone()
            .list_buildings_by_fortress(list_buildings_by_fortress_request)
            .await?
            .into_inner()
            .buildings;
        let gold_bonus = get_gold_bonus(buildings);
        let update_fortress = UpdateFortress {
            id: fortress_id,
            gold: Some(fortress.gold + 1 + gold_bonus),
            food: None,
            wood: None,
            energy: None,
        };
        let update_fortress_request = crate::pb::crud::v1::UpdateFortressRequest {
            fortress: Some(update_fortress),
        };
        let fortress = self
            .crud_fortress_client
            .clone()
            .update_fortress(update_fortress_request)
            .await?
            .into_inner()
            .fortress;
        let message = CollectFortressGoldResponse { fortress };

        Ok(Response::new(message))
    }

    async fn get_fortress_food(
        &self,
        request: Request<GetFortressFoodRequest>,
    ) -> Result<Response<GetFortressFoodResponse>, Status> {
        let get_fortress_request = crate::pb::crud::v1::GetFortressRequest {
            id: request.into_inner().id,
        };
        let food = self
            .crud_fortress_client
            .clone()
            .get_fortress(get_fortress_request)
            .await?
            .into_inner()
            .fortress
            .ok_or_else(|| Status::not_found("fortress not found"))?
            .food;
        let message = GetFortressFoodResponse { food };

        Ok(Response::new(message))
    }

    async fn collect_fortress_food(
        &self,
        request: Request<CollectFortressFoodRequest>,
    ) -> Result<Response<CollectFortressFoodResponse>, Status> {
        let fortress_id = request.into_inner().id;
        let get_fortress_request = crate::pb::crud::v1::GetFortressRequest { id: fortress_id };
        let fortress = self
            .crud_fortress_client
            .clone()
            .get_fortress(get_fortress_request)
            .await?
            .into_inner()
            .fortress
            .ok_or_else(|| Status::not_found("fortress not found"))?;
        let list_buildings_by_fortress_request =
            crate::pb::crud::v1::ListBuildingsByFortressRequest { fortress_id };
        let buildings = self
            .crud_building_client
            .clone()
            .list_buildings_by_fortress(list_buildings_by_fortress_request)
            .await?
            .into_inner()
            .buildings;
        let food_bonus = get_food_bonus(buildings);
        let update_fortress = UpdateFortress {
            id: fortress_id,
            gold: None,
            food: Some(fortress.food + 1 + food_bonus),
            wood: None,
            energy: None,
        };
        let update_fortress_request = crate::pb::crud::v1::UpdateFortressRequest {
            fortress: Some(update_fortress),
        };
        let fortress = self
            .crud_fortress_client
            .clone()
            .update_fortress(update_fortress_request)
            .await?
            .into_inner()
            .fortress;
        let message = CollectFortressFoodResponse { fortress };

        Ok(Response::new(message))
    }

    async fn get_fortress_wood(
        &self,
        request: Request<GetFortressWoodRequest>,
    ) -> Result<Response<GetFortressWoodResponse>, Status> {
        let get_fortress_request = crate::pb::crud::v1::GetFortressRequest {
            id: request.into_inner().id,
        };
        let wood = self
            .crud_fortress_client
            .clone()
            .get_fortress(get_fortress_request)
            .await?
            .into_inner()
            .fortress
            .ok_or_else(|| Status::not_found("fortress not found"))?
            .wood;
        let message = GetFortressWoodResponse { wood };

        Ok(Response::new(message))
    }

    async fn collect_fortress_wood(
        &self,
        request: Request<CollectFortressWoodRequest>,
    ) -> Result<Response<CollectFortressWoodResponse>, Status> {
        let fortress_id = request.into_inner().id;
        let get_fortress_request = crate::pb::crud::v1::GetFortressRequest { id: fortress_id };
        let fortress = self
            .crud_fortress_client
            .clone()
            .get_fortress(get_fortress_request)
            .await?
            .into_inner()
            .fortress
            .ok_or_else(|| Status::not_found("fortress not found"))?;
        let list_buildings_by_fortress_request =
            crate::pb::crud::v1::ListBuildingsByFortressRequest { fortress_id };
        let buildings = self
            .crud_building_client
            .clone()
            .list_buildings_by_fortress(list_buildings_by_fortress_request)
            .await?
            .into_inner()
            .buildings;
        let wood_bonus = get_wood_bonus(buildings);
        let update_fortress = UpdateFortress {
            id: fortress_id,
            gold: None,
            food: None,
            wood: Some(fortress.wood + 1 + wood_bonus),
            energy: None,
        };
        let update_fortress_request = crate::pb::crud::v1::UpdateFortressRequest {
            fortress: Some(update_fortress),
        };
        let fortress = self
            .crud_fortress_client
            .clone()
            .update_fortress(update_fortress_request)
            .await?
            .into_inner()
            .fortress;
        let message = CollectFortressWoodResponse { fortress };

        Ok(Response::new(message))
    }

    async fn get_fortress_energy(
        &self,
        request: Request<GetFortressEnergyRequest>,
    ) -> Result<Response<GetFortressEnergyResponse>, Status> {
        let get_fortress_request = crate::pb::crud::v1::GetFortressRequest {
            id: request.into_inner().id,
        };
        let energy = self
            .crud_fortress_client
            .clone()
            .get_fortress(get_fortress_request)
            .await?
            .into_inner()
            .fortress
            .ok_or_else(|| Status::not_found("fortress not found"))?
            .energy;

        let message = GetFortressEnergyResponse { energy };

        Ok(Response::new(message))
    }

    async fn collect_fortress_energy(
        &self,
        request: Request<CollectFortressEnergyRequest>,
    ) -> Result<Response<CollectFortressEnergyResponse>, Status> {
        let fortress_id = request.into_inner().id;
        let get_fortress_request = crate::pb::crud::v1::GetFortressRequest { id: fortress_id };
        let fortress = self
            .crud_fortress_client
            .clone()
            .get_fortress(get_fortress_request)
            .await?
            .into_inner()
            .fortress
            .ok_or_else(|| Status::not_found("fortress not found"))?;
        let list_buildings_by_fortress_request =
            crate::pb::crud::v1::ListBuildingsByFortressRequest { fortress_id };
        let buildings = self
            .crud_building_client
            .clone()
            .list_buildings_by_fortress(list_buildings_by_fortress_request)
            .await?
            .into_inner()
            .buildings;
        let energy_bonus = get_energy_bonus(buildings);
        let update_fortress = UpdateFortress {
            id: fortress_id,
            gold: None,
            food: None,
            wood: None,
            energy: Some(fortress.energy + 1 + energy_bonus),
        };
        let update_fortress_request = crate::pb::crud::v1::UpdateFortressRequest {
            fortress: Some(update_fortress),
        };
        let fortress = self
            .crud_fortress_client
            .clone()
            .update_fortress(update_fortress_request)
            .await?
            .into_inner()
            .fortress;
        let message = CollectFortressEnergyResponse { fortress };

        Ok(Response::new(message))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upgrade_building() {
        let fortress = Fortress {
            id: 0,
            gold: 0,
            food: 0,
            wood: 0,
            energy: 0,
        };
        let building = Building {
            id: 0,
            name: String::from(""),
            level: 1,
            fortress_id: 0,
        };
        let costs = Costs {
            gold: 0,
            food: 0,
            wood: 0,
            energy: 0,
        };
        let result = upgrade_building(&fortress, &building, &costs, 200);
        assert!(result.is_ok());
        if let Ok((update_fortress, update_building)) = result {
            assert_eq!(update_fortress.gold, Some(0));
            assert_eq!(update_fortress.food, Some(0));
            assert_eq!(update_fortress.wood, Some(0));
            assert_eq!(update_fortress.energy, Some(0));
            assert_eq!(update_building.level, Some(building.level + 1));
        }
    }

    #[test]
    fn test_upgrade_building_error() {
        {
            let fortress = Fortress {
                id: 0,
                gold: 10,
                food: 0,
                wood: 0,
                energy: 0,
            };
            let building = Building {
                id: 0,
                name: String::from(""),
                level: 1,
                fortress_id: 0,
            };
            let costs = Costs {
                gold: 11,
                food: 0,
                wood: 0,
                energy: 0,
            };
            let result = upgrade_building(&fortress, &building, &costs, 200);
            assert!(result.is_err());
            if let Err(msg) = result {
                println!("error: {msg}");
            }
        }
        {
            let fortress = Fortress {
                id: 0,
                gold: 10,
                food: 0,
                wood: 0,
                energy: 0,
            };
            let building = Building {
                id: 0,
                name: String::from(""),
                level: 200,
                fortress_id: 0,
            };
            let costs = Costs {
                gold: 5,
                food: 0,
                wood: 0,
                energy: 0,
            };
            let result = upgrade_building(&fortress, &building, &costs, 200);
            assert!(result.is_err());
            if let Err(msg) = result {
                println!("error: {msg}");
            }
        }
        {
            let fortress = Fortress {
                id: 1,
                gold: 10,
                food: 0,
                wood: 0,
                energy: 0,
            };
            let building = Building {
                id: 0,
                name: String::from(""),
                level: 1,
                fortress_id: 0,
            };
            let costs = Costs {
                gold: 5,
                food: 0,
                wood: 0,
                energy: 0,
            };
            let result = upgrade_building(&fortress, &building, &costs, 200);
            assert!(result.is_err());
            if let Err(msg) = result {
                println!("error: {msg}");
            }
        }
    }

    #[test]
    fn test_optimize_factor() {
        const BASE: i32 = 10;
        let cost_max: i32 = i32::MAX;
        let levels: Vec<i32> = vec![10, 50, 100, 200, 1000];

        for level in levels {
            let factor = optimize_factor(level, BASE, cost_max);
            println!("level max: {level}, base: {BASE}, factor: {factor}");
            assert!(factor >= 1.0);
        }
    }

    #[test]
    fn test_upgrade_cost() {
        const BASE: i32 = 10;
        const FACTOR: f64 = 1.1012;
        let result = upgrade_cost(0, BASE, FACTOR) as i32;
        assert_eq!(result, 10);
        let result = upgrade_cost(1, BASE, FACTOR) as i32;
        assert_eq!(result, 10);
        let result = upgrade_cost(2, BASE, FACTOR) as i32;
        assert_eq!(result, 11);
        let result = upgrade_cost(3, BASE, FACTOR) as i32;
        assert_eq!(result, 12);
        let result = upgrade_cost(4, BASE, FACTOR) as i32;
        assert_eq!(result, 13);
        let result = upgrade_cost(5, BASE, FACTOR) as i32;
        assert_eq!(result, 14);
        let result = upgrade_cost(50, BASE, FACTOR) as i32;
        assert_eq!(result, 1125);
        let result = upgrade_cost(100, BASE, FACTOR) as i32;
        assert_eq!(result, 139557);
        let result = upgrade_cost(150, BASE, FACTOR) as i32;
        assert_eq!(result, 17300721);
        let result = upgrade_cost(200, BASE, FACTOR) as i32;
        assert_eq!(result, 2144738468);
    }

    #[test]
    fn test_upgrade_cost_overflow() {
        const BASE: i32 = 10;
        const FACTOR: f64 = 1.1012;
        let result = upgrade_cost(250, BASE, FACTOR) as i32;
        assert_eq!(result, i32::MAX);
        let result = upgrade_cost(1000, BASE, FACTOR) as i32;
        assert_eq!(result, i32::MAX);
        let result = upgrade_cost(i32::MAX, BASE, FACTOR) as i32;
        assert_eq!(result, i32::MAX);
    }

    #[test]
    fn test_level_slice() {
        const BASE: i32 = 10;
        let cost_max: i32 = i32::MAX;
        let levels: Vec<i32> = vec![10, 50, 100, 200, 1000];

        for level in levels {
            let factor = optimize_factor(level, BASE, cost_max);
            let result_min = upgrade_cost(1, BASE, factor) as i32;
            let result_max = upgrade_cost(level, BASE, factor) as i32;
            println!("{}: {} -> {}: {}", 1, result_min, level, result_max);
            assert!(BASE <= result_min && result_min <= cost_max);
            assert!(BASE <= result_max && result_max <= cost_max);
            assert!(result_min <= result_max);
        }
    }
}
