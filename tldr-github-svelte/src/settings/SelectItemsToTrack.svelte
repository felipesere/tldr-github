<script>
    import {createEventDispatcher} from 'svelte';
    import Label from "./Label.svelte";

    export let repo;
    const dispatch = createEventDispatcher();

    const close = () => dispatch('close')
</script>

<div class="modal is-active">
    <div class="modal-background"></div>
    <div class="modal-card">
        <header class="modal-card-head">
            <p class="modal-card-title">Add new PRs and issues to track</p>
            <button class="delete" aria-label="close" on:click={close}></button>
        </header>
        <section class="modal-card-body">
            <table class="table is-striped is-hoverable">
                <thead>
                <tr>
                    <th>Title</th>
                    <th>By</th>
                    <th>Labels</th>
                </tr>
                </thead>
                <tbody>
                {#each repo.activity.prs as pr}
                    <tr class="fixed-height">
                        <td class="item-title">{pr.title}</td>
                        <td>{pr.by}</td>
                        <td>
                            <div class="horizontal-flex">
                                {#each pr.labels as l}
                                    <Label value={l}/>
                                {/each}
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
    .item-title {
        width: 200px;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
    }

    .modal-card {
        width: 1200px !important;
        max-height: calc(100vh - 200px) !important;
    }

    .fixed-height {
        min-height: 20px;
    }

</style>