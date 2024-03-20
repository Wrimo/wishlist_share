<script>
    import { onMount } from "svelte";
    export let person_id;
    let data = [];

    onMount(async () => {
        const response = await fetch(
            `http://127.0.0.1:8000/family/${person_id}`,
        );
        data = await response.json();
    });
</script>

<div class="bg-slate-100 rounded-xl ">
    <h1 class="p-2 text-black text-5xl">Your Family</h1>
    <div class="container m-auto grid grid-cols-12 gap-1">
        {#each data as person, i}
            <div class="col-span-12 py-2 px-4 rounded-xl">
                <button class="w-96 bg-slate-300 text-left text-black">
                    {person["first_name"] + " " + person["last_name"]}
                </button>
            </div>
        {/each}
    </div>
</div>
