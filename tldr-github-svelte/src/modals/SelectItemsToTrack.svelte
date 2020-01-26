<script>
    import {fade} from 'svelte/transition'
    import {createEventDispatcher, onMount} from 'svelte';
    import {trackItems} from '../client/api.js'
    import Label from "../atoms/Label.svelte";
    import GlowBox from "../atoms/GlowBox.svelte";
    import SearchBar from "../settings/SearchBar.svelte";
    import Spinner from "../settings/Spinner.svelte";

    export let repo;
    const dispatch = createEventDispatcher();

    let items = []

    const close = () => dispatch('close');
    let searchResults = items;

    let selected = [];

    async function track() {
        await trackItems(repo.id, {items: selected.map(s => ({kind: 'pr', nr: s.nr}))});
        close()
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

<div class="modal is-active">
    <div class="modal-background"></div>
    <div class="modal-card">
        <header class="modal-card-head">
            <p class="modal-card-title">Add new PRs and issues to track</p>
            <button class="delete" on:click={close}></button>
        </header>
        <section class="modal-card-body">
            {#await fetching}
                <Spinner/>
            {:then}
                <SearchBar items={items} bind:searchResults/>
                <table>
                    <thead>
                    <tr>
                        <th>Track?</th>
                        <th></th>
                        <th></th>
                    </tr>
                    </thead>
                    <tbody>
                    {#each searchResults as pr (pr.nr)}
                        <tr out:fade="{{duration: 250}}">
                            <td class="w-100">
                                <div class="horizontal-flex">
                                    <input type="checkbox" bind:group={selected} value={pr}/>
                                </div>
                            </td>
                            <td>
                                <GlowBox content={pr}/>
                            </td>
                            <td class="w-200">
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
        <footer class="modal-card-foot">
            <button class="button is-info" on:click={track}>Save changes</button>
            <button class="button" on:click={close}>Cancel</button>
        </footer>
    </div>
</div>

<style>
    .modal-card {
        width: 900px !important;
        height: calc(100vh - 250px);
    }
</style>