<script>
    import { onMount } from "svelte";
    import { selectedPerson } from "./store.js";
    let data = [];
    onMount(async () => {
        console.log($selectedPerson);
        if (Object.keys($selectedPerson).length === 0) { // if the page has been refreshed, go back home
            document.location.href = "/";
        }
        const response = await fetch(
            `http://127.0.0.1:8000/wishlist/${$selectedPerson["person_id"]}`,
        );
        data = await response.json();
    });
</script>

<div class="h-screen w-full flex justify-center items-center">
    <div
        class="h-max w-max bg-slate-100 rounded-xl container m-auto grid grid-cols-4 gap-1 scale-150"
    >
        <h1 class="p-2 text-black text-2xl col-span-12 m-auto">
            {$selectedPerson["first_name"]}'s Wishlist
        </h1>
            {#if data.length === 0}
                <p class="p-2 text-black text-2xl col-span-12 m-auto">This person hasn't filled out their wishlist yet!</p>
            {:else}
                {#each data as item}
                        <button class="col-span-12 h-max m-1 py-1 px-4 rounded-xl bg-slate-300 text-left text-black text-xl " class:bg-emerald-400={item["purchased"]} class:bg-slate-300={!item["purchased"]}>
                            {item["name"]}
                            <p class="text-sm">{item["description"]}</p>
                        </button>
                {/each}
            {/if}
    </div>
</div>
