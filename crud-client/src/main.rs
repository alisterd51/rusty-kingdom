#[allow(clippy::pedantic)]
#[allow(clippy::nursery)]
pub mod pb {
    pub mod common {
        pub mod v1 {
            tonic::include_proto!("common.v1");
        }
    }
    pub mod crud {
        pub mod v1 {
            tonic::include_proto!("crud.v1");
        }
    }
}

use pb::crud::v1::{ListFortressesRequest, fortress_service_client::FortressServiceClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = FortressServiceClient::connect("http://localhost:3000").await?;

    // for _ in 0..1000 {
    //     let request = tonic::Request::new(CreateFortressRequest {
    //         fortress: Some(NewFortress {
    //             gold: 0,
    //             food: 0,
    //             wood: 0,
    //             energy: 0,
    //         }),
    //     });

    //     let _response = client.create_fortress(request).await?;
    // }
    // println!("{:?}", response.into_inner().fortress);

    let response = client
        .list_fortresses(tonic::Request::new(ListFortressesRequest {}))
        .await?;
    println!("{:?}", response.into_inner().fortresses);

    Ok(())
}
