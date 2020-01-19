<script>
    import {fade} from 'svelte/transition'
    import {createEventDispatcher} from 'svelte';
    import Label from "./Label.svelte";
    import GlowBox from "../GlowBox.svelte";
    import SearchBar from "./SearchBar.svelte";

    export let repo;
    const dispatch = createEventDispatcher();

    const close = () => dispatch('close');
    let searchResults = repo.activity.prs
</script>

<div class="modal is-active">
    <div class="modal-background"></div>
    <div class="modal-card">
        <header class="modal-card-head">
            <p class="modal-card-title">Add new PRs and issues to track</p>
            <button class="delete" aria-label="close" on:click={close}></button>
        </header>
        <section class="modal-card-body">
            <SearchBar items={repo.activity.prs} fields={["title", "by", "labels"]} bind:searchResults/>
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
                                <input type="checkbox" name="track">
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
            <button class="button is-success">Save changes</button>
            <button class="button" on:click={close}>Cancel</button>
        </footer>
    </div>
</div>

<style>
    .modal-card {
        width: 900px !important;
        min-height: calc(100vh - 250px);
    }
</style>