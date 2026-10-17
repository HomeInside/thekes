//
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveModule {
    //Home
    Dashboard,
    Books,
    Authors,
    Notes,
    //Library
    Unclassified,
    Favorites,
    CurrentlyReading,
    WantToRead,
    Finished,
}

pub struct NavigateTo {
    pub module: ActiveModule,
}
