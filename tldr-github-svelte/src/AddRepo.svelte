<script>
  import { createEventDispatcher } from 'svelte';
  const dispatch = createEventDispatcher();

  let newRepoName = "";
  let currentlyAddingRepo

  function handleClick() {
    (currentlyAddingRepo = async () => {
      await post("/repos", {name: newRepoName})
      setTimeout(() => {
        currentlyAddingRepo = undefined
        dispatch('new-repo-added')
        newRepoName = ""
      }, 500)
    })()
  }

  async function post(path, data) {
    await fetch(`http://localhost:8080/api${path}`, {
      "body": JSON.stringify(data),
      "method": "POST",
      "headers": {
        "Content-Type": "application/json",
      },
    })
  };
</script>

<article class="card horizontal-flex">
  <div class="card-content content grow">
    <div class="field has-addons">
      <div class="control has-icons-right grow">
        <input bind:value={newRepoName} class="input" type="text" placeholder="Add new repo" />
        <span class="icon is-small is-right">
          <i class="icon ion-md-checkmark" />
        </span>
      </div>
      <div class="control">
        <button on:click|preventDefault={handleClick} disabled={newRepoName === "" || currentlyAddingRepo} class="button is-info">Add</button>
      </div>
    </div>
  </div>
</article>
