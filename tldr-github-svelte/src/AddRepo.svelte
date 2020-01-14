<script>
  import { createEventDispatcher } from 'svelte';
  import {newError} from './errors/errorStore.js';
  const dispatch = createEventDispatcher();
  let newRepoName = "";
  let currentlyAddingRepo

  function handleClick() {
    (currentlyAddingRepo = async () => {
      let response = await post("/repos", {name: newRepoName})
      if (!response.ok) {
        newError(`Could not add repo ${newRepoName}`);
        newRepoName = ""
        currentlyAddingRepo = undefined
      } else {
        setTimeout(() => {
          currentlyAddingRepo = undefined
          dispatch('new-repo-added')
          newRepoName = ""
        }, 500)
      }
    })()
  };

  async function post(path, data) {
    return await fetch(`/api${path}`, {
      "body": JSON.stringify(data),
      "method": "POST",
      "headers": {
        "Content-Type": "application/json",
      },
    })
  };
</script>

<article class="vertical-flex at-most-450">
  <div class="content">
    <form class="field has-addons">
      <div class="control has-icons-right grow">
        <input bind:value={newRepoName} class="input" type="text" placeholder="Add new repo" />
        <span class="icon is-small is-right">
          <i class="icon ion-md-checkmark" />
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
