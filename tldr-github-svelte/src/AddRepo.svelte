<script>
    import {createEventDispatcher} from 'svelte';
    import {addRepo} from './client/api.js';

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
</script>

<article class="vertical-flex at-most-450">
    <div class="content">
        <form class="field has-addons">
            <div class="control has-icons-right grow">
                <input bind:value={newRepoName} class="input" type="text" placeholder="Add new repo"/>
                <span class="icon is-small is-right">
          <i class="icon ion-md-checkmark"></i>
        </span>
            </div>
            <div class="control">
                <button
                        on:click|preventDefault={handleClick}
                        disabled={newRepoName === "" || currentlyAddingRepo }
                        class:is-loading={currentlyAddingRepo}
                        class="button is-info">
                    Add
                </button>
            </div>
        </form>
    </div>
</article>

<style>
    .at-most-450 {
        max-width: 450px;
    }
</style>
