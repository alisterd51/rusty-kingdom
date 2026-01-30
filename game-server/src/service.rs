use crate::pb::{
    common::v1::{Costs, NewBuilding, NewFortress},
    crud::v1::{
        CollectFortressResourceRequest, ResourceKind, UpgradeBuildingAtomicRequest,
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
const BASE_COST: i32 = 10;
const GOLD_BONUS_BUILDING: &str = "bank";
const FOOD_BONUS_BUILDING: &str = "farm";
const WOOD_BONUS_BUILDING: &str = "sawmill";
const ENERGY_BONUS_BUILDING: &str = "sanctuary";

fn upgrade_cost(level: i32, base: i32, factor: f64) -> f64 {
    let level = level.max(1);
    f64::from(base) * factor.powf(f64::from(level - 1))
}

fn optimize_factor(level_max: i32, base: i32, cost_max: i32) -> f64 {
    (f64::from(cost_max) / f64::from(base)).powf(1.0 / (f64::from(level_max) - 1.0))
}

#[allow(clippy::cast_possible_truncation)]
fn get_costs(level: i32, level_max: i32) -> Costs {
    let factor = optimize_factor(level_max, BASE_COST, i32::MAX);
    let cost = upgrade_cost(level, BASE_COST, factor) as i32;

    Costs {
        gold: cost / 2,
        food: cost,
        wood: cost / 5,
        energy: cost / 10,
    }
}

pub struct MyBuildingService {
    crud_building_client: BuildingServiceClient<tonic::transport::Channel>,
}

impl MyBuildingService {
    pub const fn new(
        crud_building_client: BuildingServiceClient<tonic::transport::Channel>,
    ) -> Self {
        Self {
            crud_building_client,
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
        let response = GetBuildingResponse {
            building: building.into_inner().building,
        };
        Ok(Response::new(response))
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
        let building = self
            .crud_building_client
            .clone()
            .get_building(Request::new(crate::pb::crud::v1::GetBuildingRequest {
                id: building_id,
            }))
            .await?
            .into_inner()
            .building
            .ok_or_else(|| Status::not_found("building not found"))?;
        let costs = get_costs(building.level, MAX_BUILDING_LEVEL);
        let upgrade_req = UpgradeBuildingAtomicRequest {
            building_id,
            costs: Some(costs),
            expected_building_level: Some(building.level),
            max_building_level: MAX_BUILDING_LEVEL,
        };
        let upgraded = self
            .crud_building_client
            .clone()
            .upgrade_building_atomic(Request::new(upgrade_req))
            .await?
            .into_inner();

        Ok(Response::new(ImproveBuildingResponse {
            fortress: upgraded.fortress,
            building: upgraded.building,
        }))
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
        let new_buildings = [
            NewBuilding {
                name: GOLD_BONUS_BUILDING.to_string(),
                level: 0,
                fortress_id: fortress.id,
            },
            NewBuilding {
                name: FOOD_BONUS_BUILDING.to_string(),
                level: 0,
                fortress_id: fortress.id,
            },
            NewBuilding {
                name: WOOD_BONUS_BUILDING.to_string(),
                level: 0,
                fortress_id: fortress.id,
            },
            NewBuilding {
                name: ENERGY_BONUS_BUILDING.to_string(),
                level: 0,
                fortress_id: fortress.id,
            },
        ];
        let mut buildings = Vec::with_capacity(new_buildings.len());
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
        let collect_request = CollectFortressResourceRequest {
            id: request.into_inner().id,
            resource: ResourceKind::Gold as i32,
            bonus_building_name: GOLD_BONUS_BUILDING.to_string(),
            base: None,
        };
        let fortress = self
            .crud_fortress_client
            .clone()
            .collect_fortress_resource(collect_request)
            .await?
            .into_inner()
            .fortress;

        Ok(Response::new(CollectFortressGoldResponse { fortress }))
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
        let collect_request = CollectFortressResourceRequest {
            id: request.into_inner().id,
            resource: ResourceKind::Food as i32,
            bonus_building_name: FOOD_BONUS_BUILDING.to_string(),
            base: None,
        };
        let fortress = self
            .crud_fortress_client
            .clone()
            .collect_fortress_resource(collect_request)
            .await?
            .into_inner()
            .fortress;

        Ok(Response::new(CollectFortressFoodResponse { fortress }))
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
        let collect_request = CollectFortressResourceRequest {
            id: request.into_inner().id,
            resource: ResourceKind::Wood as i32,
            bonus_building_name: WOOD_BONUS_BUILDING.to_string(),
            base: None,
        };
        let fortress = self
            .crud_fortress_client
            .clone()
            .collect_fortress_resource(collect_request)
            .await?
            .into_inner()
            .fortress;

        Ok(Response::new(CollectFortressWoodResponse { fortress }))
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
        let collect_request = CollectFortressResourceRequest {
            id: request.into_inner().id,
            resource: ResourceKind::Energy as i32,
            bonus_building_name: ENERGY_BONUS_BUILDING.to_string(),
            base: None,
        };
        let fortress = self
            .crud_fortress_client
            .clone()
            .collect_fortress_resource(collect_request)
            .await?
            .into_inner()
            .fortress;

        Ok(Response::new(CollectFortressEnergyResponse { fortress }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimize_factor() {
        let cost_max = i32::MAX;
        let levels = [10, 50, 100, 200, 1000];

        for level in levels {
            let factor = optimize_factor(level, BASE_COST, cost_max);
            println!("level max: {level}, base: {BASE_COST}, factor: {factor}");
            assert!(factor >= 1.0);
        }
    }

    #[test]
    fn test_upgrade_cost() {
        const FACTOR: f64 = 1.1012;
        let result = upgrade_cost(0, BASE_COST, FACTOR) as i32;
        assert_eq!(result, 10);
        let result = upgrade_cost(1, BASE_COST, FACTOR) as i32;
        assert_eq!(result, 10);
        let result = upgrade_cost(2, BASE_COST, FACTOR) as i32;
        assert_eq!(result, 11);
        let result = upgrade_cost(3, BASE_COST, FACTOR) as i32;
        assert_eq!(result, 12);
        let result = upgrade_cost(4, BASE_COST, FACTOR) as i32;
        assert_eq!(result, 13);
        let result = upgrade_cost(5, BASE_COST, FACTOR) as i32;
        assert_eq!(result, 14);
        let result = upgrade_cost(50, BASE_COST, FACTOR) as i32;
        assert_eq!(result, 1125);
        let result = upgrade_cost(100, BASE_COST, FACTOR) as i32;
        assert_eq!(result, 139557);
        let result = upgrade_cost(150, BASE_COST, FACTOR) as i32;
        assert_eq!(result, 17300721);
        let result = upgrade_cost(200, BASE_COST, FACTOR) as i32;
        assert_eq!(result, 2144738468);
    }

    #[test]
    fn test_upgrade_cost_overflow() {
        const FACTOR: f64 = 1.1012;
        let result = upgrade_cost(250, BASE_COST, FACTOR) as i32;
        assert_eq!(result, i32::MAX);
        let result = upgrade_cost(1000, BASE_COST, FACTOR) as i32;
        assert_eq!(result, i32::MAX);
        let result = upgrade_cost(i32::MAX, BASE_COST, FACTOR) as i32;
        assert_eq!(result, i32::MAX);
    }

    #[test]
    fn test_level_slice() {
        let cost_max = i32::MAX;
        let levels = [10, 50, 100, 200, 1000];

        for level in levels {
            let factor = optimize_factor(level, BASE_COST, cost_max);
            let result_min = upgrade_cost(1, BASE_COST, factor) as i32;
            let result_max = upgrade_cost(level, BASE_COST, factor) as i32;
            println!("{}: {} -> {}: {}", 1, result_min, level, result_max);
            assert!(BASE_COST <= result_min && result_min <= cost_max);
            assert!(BASE_COST <= result_max && result_max <= cost_max);
            assert!(result_min <= result_max);
        }
    }
}
