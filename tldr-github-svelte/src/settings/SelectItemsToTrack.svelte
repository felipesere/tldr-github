<script>
    import {fade} from 'svelte/transition'
    import {createEventDispatcher} from 'svelte';
    import { trackItems } from '../client/api.js'
    import Label from "./Label.svelte";
    import GlowBox from "../GlowBox.svelte";
    import SearchBar from "./SearchBar.svelte";

    export let repo;
    const dispatch = createEventDispatcher();

    const close = () => dispatch('close');
    let searchResults = [...repo.activity.prs, ...repo.activity.issues];

    let selected = [];

    async function track() {
       await trackItems(repo.id, {items: selected.map(s => ({kind: 'pr', nr: s.nr}))});
       close()
    }
</script>

<div class="modal is-active">
    <div class="modal-background"></div>
    <div class="modal-card">
        <header class="modal-card-head">
            <p class="modal-card-title">Add new PRs and issues to track</p>
            <button class="delete" aria-label="close" on:click={close}></button>
        </header>
        <section class="modal-card-body">
            <SearchBar items={[...repo.activity.prs, ...repo.activity.issues]} bind:searchResults/>
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
                                <input type="checkbox" bind:group={selected} value={pr} />
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
        </section>
        <footer class="modal-card-foot">
            <button class="button is-success" on:click={track}>Save changes</button>
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