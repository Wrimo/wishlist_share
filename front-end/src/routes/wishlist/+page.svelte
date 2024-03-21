<script>
    import WishlistView from "../../lib/WishlistView.svelte";
    import { page } from "$app/stores";
    const person_id = $page.url.searchParams.get("person_id");
</script>

<WishlistView />
