<script>
    import {createEventDispatcher} from 'svelte';

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
                    <th>Nr</th>
                    <th>Title</th>
                    <th>By</th>
                </tr>
                </thead>
                <tbody>
                {#each repo.activity.prs as pr}
                    <tr>
                        <td>1</td>
                        <td>{pr.title}</td>
                        <td>{pr.by}</td>
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
</style>