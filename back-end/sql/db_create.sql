create table Person ( 
    id integer primary key, 
    first_name text, 
    last_name text, 
    email text
);

create table Family (
    id integer primary key, 
    name text 
);

create table FamilyPerson ( 
    id integer primary key, 
    person_id integer references Person(id),
    family_id integer references Family(id) 
);

create table Wishlist ( 
    id integer primary key, 
    person_id integer references Person(id), 
    comment text 
);

create table WishlistItem ( 
    id integer primary key, 
    wishlist_id integer references Wishlist(id), 
    name text, 
    description text
);