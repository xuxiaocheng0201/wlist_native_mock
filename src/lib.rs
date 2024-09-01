pub mod api;

mod frb_generated;
#[allow(unused_imports)]
mod fix_frb_generated {
    pub use super::api::common::data::*;
    pub use super::api::common::data::storages::*;
    pub use super::api::common::data::storages::information::*;
    pub use super::api::common::data::storages::options::*;
    pub use super::api::common::data::files::*;
    pub use super::api::common::data::files::tokens::*;
    pub use super::api::common::data::files::confirmations::*;
    pub use super::api::common::data::files::information::*;
    pub use super::api::common::data::files::progresses::*;
    pub use super::api::common::data::files::options::*;
    pub use super::api::common::data::trashes::*;
    pub use super::api::common::data::trashes::information::*;
    pub use super::api::common::data::trashes::options::*;
    pub use super::api::common::exceptions::*;
}
use fix_frb_generated::*;
