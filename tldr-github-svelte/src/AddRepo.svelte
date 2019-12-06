<script>
  import { createEventDispatcher } from 'svelte';
	const dispatch = createEventDispatcher();

  let newRepoName = "";

  // $: newRepoName, console.log(newRepoName);
  function handleClick() {
    dispatch('new-repo', {name: newRepoName})
  }

  function empty(name) {
    return name === ""
  }

  
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
        <button on:click|preventDefault={handleClick} disabled={newRepoName === ""} class="button is-info">Add</button>
      </div>
    </div>
  </div>
</article>
