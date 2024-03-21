<script>
    import { onMount } from "svelte";
    import { selectedPerson } from "./store.js";
    let data = [];
    onMount(async () => {
        console.log($selectedPerson)
        if (Object.keys($selectedPerson).length === 0) {
            document.location.href = "/";
        }
        const response = await fetch(
            `http://127.0.0.1:8000/wishlist/${$selectedPerson["person_id"]}`,
        );
        data = await response.json();
    });
</script>

<div class="bg-slate-100 rounded-xl">
    <h1 class="p-2 text-black text-5xl">Name's Wishlist</h1>
    <div class="container m-auto grid grid-cols-12 gap-1">
        {#if data.length === 0}
            <p>This person hasn't filled out their wishlist yet!</p>
        {:else}
            {#each data as item}
                <div
                    class="col-span-12 py-2 px-4 rounded-xl"
                    class:bg-emerald-400={item["purchased"]}
                    class:bg-slate-300={!item["purchased"]}
                >
                    <button class="w-96 text-left text-black">
                        {item["name"]}
                        <h3>{item["description"]}</h3>
                    </button>
                </div>
            {/each}
        {/if}
    </div>
</div>
