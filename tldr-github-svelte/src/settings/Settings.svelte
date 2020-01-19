<script>
    import {deleteRepo} from '../client/api.js'
    import {createEventDispatcher} from 'svelte';
    import SelectItemsToTrack from "./SelectItemsToTrack.svelte";

    const dispatch = createEventDispatcher();

    export let repo;
    let currentlyDeletingRepo;

    function deleteThisRepo() {
        (currentlyDeletingRepo = async () => {
            try {
                await deleteRepo(repo.id);
                dispatch('repo-deleted');
            } catch (e) {
            }
            currentlyDeletingRepo = undefined
        })()
    }

    let isModalOpen = false;
    const openModal = () => isModalOpen = true;
    const closeModal = () => isModalOpen = false;
</script>

<section class="content stack">
    <div class="horizontal-flex">
        <button class="button is-normal" class:is-loading={currentlyDeletingRepo}
                on:click|preventDefault={deleteThisRepo}>
            Delete
        </button>
        <p class="grow is-normal">to stop tracking this repo</p>
    </div>
    <div class="horizontal-flex">
        <button class="button is-normal" class:is-loading={false} on:click|preventDefault={openModal}>
            Select
        </button>
        <p class="grow is-normal">issues and pull request</p>
    </div>
    {#if isModalOpen }
        <SelectItemsToTrack repo={repo} on:close={closeModal}/>
    {/if}
</section>

<style>

    p {
        margin-top: auto;
        margin-bottom: auto !important;
        padding-left: 5px;
    }
</style>
