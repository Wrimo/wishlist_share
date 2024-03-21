begin; 

CREATE OR REPLACE FUNCTION get_person_wishlist_items(person_id integer)
RETURNS SETOF WishlistItem AS $$
select 
    WishlistItem.id, 
    WishlistItem.wishlist_id,
    WishlistItem.name,
    WishlistItem.description, 
    WishlistItem.link,
    WishlistItem.purchased
from 
    Person as p 
    join Wishlist on Wishlist.person_id = p.id 
    join WishlistItem on WishlistItem.wishlist_id = Wishlist.id
where 
    p.id = $1;
$$ LANGUAGE SQL; 

CREATE OR REPLACE FUNCTION get_person_family(person_id integer)
RETURNS TABLE(person_id integer, first_name text, last_name text) AS $$
select 
    p2.id, 
    p2.first_name, 
    p2.last_name 
from 
    Person as p 
    join FamilyPerson fp on fp.person_id = p.id and p.id = $1
    join FamilyPerson fp2 on fp2.family_id = fp.family_id
    join Person p2 on p2.id = fp2.person_id
where p2.id != $1;
$$ LANGUAGE SQL;

CREATE OR REPLACE PROCEDURE set_purchased(item_id integer)
LANGUAGE SQL AS $$
update WishlistItem set purchased = not purchased 
where id = $1;
$$;

commit; 