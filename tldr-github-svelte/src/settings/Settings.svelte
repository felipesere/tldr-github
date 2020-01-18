<script>
    import {deleteRepo} from '../client/api.js'
    import {createEventDispatcher} from 'svelte';

    const dispatch = createEventDispatcher();

    export let repoId;
    let currentlyDeletingRepo;

    function handleClick() {
        (currentlyDeletingRepo = async () => {
            try {
                await deleteRepo(repoId);
                dispatch('repo-deleted');
            } catch (e) {
            }
            currentlyDeletingRepo = undefined
        })()
    }
</script>

<section class="content">
    <div class="horizontal-flex">
        <button class="button is-normal" class:is-loading={currentlyDeletingRepo} on:click|preventDefault={handleClick}>
            Delete
        </button>
        <p class="grow is-normal">to stop tracking this repo</p>
        <div>
</section>

<style>

    p {
        margin-top: auto;
        margin-bottom: auto !important;
        padding-left: 5px;
    }
</style>
