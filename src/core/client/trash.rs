use either::Either;

use crate::common::data::files::confirmations::RefreshConfirmation;
use crate::common::data::files::FileLocation;
use crate::common::data::files::information::FileInformation;
use crate::common::data::trashes::information::{TrashDetailsInformation, TrashInformation, TrashListInformation};
use crate::common::data::trashes::options::ListTrashOptions;
use crate::core::client::context::define_func;

define_func!(trash_list(login_context, storage: i64, options: ListTrashOptions) -> Either<TrashListInformation, RefreshConfirmation> = {
    unimplemented!()
});
define_func!(trash_refresh(login_context, storage: i64) -> RefreshConfirmation = {
    unimplemented!()
});
define_func!(trash_get(login_context, location: FileLocation, check: bool) -> TrashDetailsInformation = {
    unimplemented!()
});
define_func!(trash_trash(login_context, location: FileLocation) -> TrashInformation = {
    unimplemented!()
});
define_func!(trash_restore(login_context, location: FileLocation, parent: i64) -> FileInformation = {
    unimplemented!()
});
define_func!(trash_delete(login_context, location: FileLocation) -> () = {
    unimplemented!()
});
define_func!(trash_delete_all(login_context, storage: i64) -> () = {
    unimplemented!()
});
