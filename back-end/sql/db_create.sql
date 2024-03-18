begin; 

create table Person ( 
    id serial primary key, 
    first_name text not null, 
    last_name text not null, 
    email text
);

create table Family (
    id serial primary key, 
    name text 
);

create table FamilyPerson ( 
    id serial primary key, 
    person_id integer references Person(id),
    family_id integer references Family(id) 
);

create table Wishlist ( 
    id serial primary key, 
    person_id integer references Person(id), 
    comment text 
);

create table WishlistItem ( 
    id serial primary key, 
    wishlist_id integer references Wishlist(id), 
    name text not null, 
    description text,
    link text, 
    purchased bool
);


insert into Family values (1, 'Cottrell  Family');

insert into Person values 
(1, 'Brennan', 'Cottrell', null),
(2, 'Alec', 'Cottrell', null),
(3, 'Erin', 'Hart', null),
(4, 'Dani', 'Self', null);

insert into FamilyPerson values 
(1, 1, 1), 
(2, 2, 1), 
(3, 3, 1), 
(4, 4, 1);

insert into wishlist(person_id, comment) values (1, 'Get me something good!');

insert into wishlistitem values 
(1, 1, 'Swiss Army Backpack', 'A cool backpack I could use for work', null, false),
(2, 1, 'Bookshelves', 'For books', null, false);

commit; 