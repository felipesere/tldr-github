<script>
    import {fade} from 'svelte/transition'
    import {createEventDispatcher, onMount} from 'svelte';
    import {trackItems} from '../client/api.js'
    import Label from "../atoms/Label.svelte";
    import GlowBox from "../atoms/GlowBox.svelte";
    import SearchBar from "../settings/SearchBar.svelte";
    import Spinner from "../settings/Spinner.svelte";
    import Indicator from "../atoms/Indicator.svelte";

    export let repo;
    const dispatch = createEventDispatcher();

    let items = []

    const close = () => dispatch('close');
    let searchResults = items;

    let selected = [];

    async function track() {
        await trackItems(repo.id, {items: selected.map(s => ({kind: s.kind, nr: s.nr}))});
        close();
        dispatch('repo-updated')
    }

    const fetchItems = async () => {
        const response = await fetch(`/api/repos/${repo.id}/proxy`);
        items = await response.json();
    };

    let fetching;

    onMount(async () => {
        fetching = fetchItems();
        await fetching;
        fetching = undefined
    })

</script>

<div class="flex flex-col items-center fixed top-0 left-0 right-0 bottom-0">
    <div class="z-20 fixed top-0 left-0 right-0 bottom-0 bg-black opacity-75"></div>
    <div class="z-30 mt-12 fixed-size rounded-lg flex flex-col overflow-scroll bg-white">
        <header class="border-b-2 shadow bg-gray-200 flex flex-row p-4">
            <p class="modal-card-title">Add new PRs and issues to track</p>
        </header>
        <section class="bg-white overflow-auto p-5 flex-grow">
            {#await fetching}
                <Spinner/>
            {:then}
                <SearchBar items={items} bind:searchResults/>
                <table class="table-auto mt-4">
                    <thead>
                    <tr>
                        <th class="px-4 py-2 font-light">Track?</th>
                        <th class="px-4 py-2 font-light">Title</th>
                        <th class="px-4 py-2 font-light">Lables</th>
                    </tr>
                    </thead>
                    <tbody>
                    {#each searchResults as pr (pr.nr)}
                        <tr out:fade="{{duration: 250}}" class="">
                            <td class="border-t p-4">
                                <input type="checkbox" bind:group={selected} value={pr}/>
                            </td>
                            <td class="border-t p-4">
                                <div class="flex flex-row">
                                    <Indicator time={pr.last_updated}/>
                                    <GlowBox content={pr}/>
                                </div>
                            </td>
                            <td class="border-t p-4">
                                <div class="cluster">
                                    <div>
                                        {#each pr.labels as l}
                                            <Label value={l}/>
                                        {/each}
                                    </div>
                                </div>
                            </td>
                        </tr>
                    {/each}
                    </tbody>
                </table>
            {/await}
        </section>
        <footer class="border-t-2 shadow bg-gray-200 flex flex-row p-4">
            <button class="btn bg-blue-600 text-white mr-2" on:click={track}>Save changes</button>
            <button class="btn-normal" on:click={close}>Cancel</button>
        </footer>
    </div>
</div>

<style>
    .fixed-size {
        width: 900px;
        height: calc(100vh - 250px);
    }
</style>