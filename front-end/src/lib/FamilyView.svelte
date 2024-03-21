<script>
    import { onMount } from "svelte";
    import {selectedPerson} from "./store.js";

    export let person_id;
    let data = [];  
    onMount(async () => {
        const response = await fetch(
            `http://127.0.0.1:8000/family/${person_id}`,
        );
        data = await response.json();
    });

</script>

<div class="h-screen w-full flex justify-center items-center">
    <div
        class="h-max w-96 bg-slate-100 rounded-xl container m-auto grid grid-cols-4 gap-1 scale-150"
    >
        <h1 class="p-2 text-black text-5xl col-span-12 m-auto">Your Family</h1>
        {#each data as person}
            <a
                class="btn col-span-12 m-4 py-1 px-4 rounded-xl bg-slate-300 text-left text-black"
                href="/wishlist"
                on:click={() => $selectedPerson = person}
            >
                {person["first_name"] + " " + person["last_name"]}
            </a>
        {/each}
    </div>
</div>
