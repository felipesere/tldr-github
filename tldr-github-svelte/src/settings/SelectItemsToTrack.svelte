<script>
    import {createEventDispatcher} from 'svelte';
    import Label from "./Label.svelte";
    import GlowBox from "../GlowBox.svelte";

    export let repo;
    const dispatch = createEventDispatcher();

    const close = () => dispatch('close');

</script>

<div class="modal is-active">
    <div class="modal-background"></div>
    <div class="modal-card">
        <header class="modal-card-head">
            <p class="modal-card-title">Add new PRs and issues to track</p>
            <button class="delete" aria-label="close" on:click={close}></button>
        </header>
        <section class="modal-card-body">
            <table>
                <thead>
                <tr>
                    <th>Track?</th>
                    <th></th>
                    <th></th>
                </tr>
                </thead>
                <tbody>
                {#each repo.activity.prs as pr}
                    <tr>
                        <td>
                            <div class="horizontal-flex">
                                <input type="checkbox" name="track">
                            </div>
                        </td>
                        <td>
                            <GlowBox content={pr}/>
                        </td>
                        <td>
                            <div class="cluster">
                                <div>
                                    {#each pr.labels as l}
                                        <Label value={l}/>
                                    {/each}
                                </div>
                            </div>
                        </td>
                    </tr>
                {/each}}
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
    }
</style>