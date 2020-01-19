<script>
    import {search} from './search'

    export let items;
    export let fields;
    export let searchResults = items;
    let term = "";

    $: searchTerm = new RegExp(term, "i");
    $: searchResults = search(items, searchTerm, fields)
</script>

<div>
    <form class="field" on:submit|preventDefault={() => {}}>
        <div class="control">
            <input bind:value={term} class="input" type="text" placeholder="Search..." />
        </div>
    </form>
</div>