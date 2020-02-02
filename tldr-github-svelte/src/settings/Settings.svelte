<script>
    import {deleteRepo} from '../client/api.js'
    import {createEventDispatcher} from 'svelte';
    import SelectItemsToTrack from "../modals/SelectItemsToTrack.svelte";

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

<section class="stack">
    <div class="flex flex-row">
        <button class="btn-normal" class:is-loading={currentlyDeletingRepo}
                on:click|preventDefault={deleteThisRepo}>
            Delete
        </button>
        <p class="flex-grow my-auto pl-2">to stop tracking this repo</p>
    </div>
    <div class="flex flex-row">
        <button class="btn-normal" class:is-loading={false} on:click|preventDefault={openModal}>
            Select
        </button>
        <p class="flex-grow my-auto pl-2">issues and pull request</p>
    </div>
</section>
{#if isModalOpen }
    <SelectItemsToTrack repo={repo} on:close={closeModal} on:repo-updated />
{/if}
