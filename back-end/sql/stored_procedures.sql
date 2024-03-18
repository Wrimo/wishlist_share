begin; 

CREATE OR REPLACE FUNCTION get_person_wishlist_items(person_id integer)
RETURNS TABLE(wishlist_id integer, name text, description text, link text, purchased bool) AS $$
select 
    Wishlist.id, 
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

commit; 