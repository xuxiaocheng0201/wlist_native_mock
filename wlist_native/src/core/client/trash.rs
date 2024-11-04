use either::Either;

use crate::common::data::files::confirmations::RefreshConfirmation;
use crate::common::data::files::FileLocation;
use crate::common::data::files::information::FileInformation;
use crate::common::data::trashes::information::{TrashDetailsInformation, TrashInformation, TrashListInformation};
use crate::common::data::trashes::options::ListTrashOptions;
use crate::core::client::define_func;

define_func!(trash_list(storage: i64, options: ListTrashOptions) -> Either<TrashListInformation, RefreshConfirmation>);
define_func!(trash_refresh(storage: i64) -> RefreshConfirmation);
define_func!(trash_get(location: FileLocation, check: bool) -> TrashDetailsInformation);
define_func!(trash_trash(location: FileLocation) -> TrashInformation);
define_func!(trash_restore(location: FileLocation, parent: i64) -> FileInformation);
define_func!(trash_delete(location: FileLocation) -> ());
define_func!(trash_delete_all(storage: i64) -> ());
