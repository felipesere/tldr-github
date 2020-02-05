<script>
    import {createEventDispatcher} from 'svelte';
    import {addRepo} from '../client/api.js';

    const dispatch = createEventDispatcher();
    let newRepoName = "";
    let currentlyAddingRepo;

    function handleClick() {
        (currentlyAddingRepo = async () => {
            try {
                await addRepo(newRepoName);
                dispatch('new-repo-added');
                newRepoName = ""
            } catch (e) {
            }
            currentlyAddingRepo = undefined;
        })()
    }

    const close = () => {
        dispatch('close')
    }
</script>

<form class="w-full bg-blue-600 border-blue-900 rounded-lg shadow-2xl">
    <div class="flex items-center border-b border-b-2 p-4">
        <input bind:value={newRepoName} class="appearance-none bg-white border-none w-full text-gray-700 mr-3 py-1 px-2 leading-tight focus:outline-none" type="text" placeholder="Example: foo/bar">
        <button class="btn-primary hover:text-blue-600 hover:bg-gray-100" type="button"
                on:click|preventDefault={handleClick}
                disabled={newRepoName === "" || currentlyAddingRepo }>
            Add
        </button>
        <button on:click={close} class="btn-normal bg-gray-200 text-gray-500 hover:text-gray-600" type="button">
            Cancel
        </button>
    </div>
</form>

<style>
    input::placeholder {
        @apply text-gray-400
    }
</style>
