use sea_query::Iden;

#[derive(Iden, Clone)]
pub enum Products {
    Table,
    Id,
    Name,
    Currency,
    Price,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
