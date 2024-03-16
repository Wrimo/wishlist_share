CREATE OR REPLACE FUNCTION get_person_wishlist_items(first_name text, last_name text)
RETURNS TABLE(name text, description text) AS $$
select 
    name,
    description
from 
    Person as p 
    join Wishlist on Wishlist.person_id = p.id 
    join WishlistItem on WishlistItem.wishlist_id = Wishlist.id
where 
    p.first_name = $1 and p.last_name = $2;
$$ LANGUAGE SQL; 