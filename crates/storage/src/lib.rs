mod filesystem;
mod inmemory;

use directip::Message;
use filesystem::FileSystemStorage;
use inmemory::VolatileStorage;

// Feature sqlite
#[cfg(feature = "sqlite")]
mod sqlite;

trait Storage {
    //fn connect(cfg: String) -> Self;
    fn save(&mut self, msg: Message) -> ();
}

enum Database {
    M(VolatileStorage),
    F(FileSystemStorage),
    #[cfg(feature = "sqlite")]
    L(sqlite::SQLiteStorage),
}
