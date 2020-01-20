<script>
    import {search} from './search'

    export let items;
    export let searchResults = items;
    let fields = ["title", "by", "labels"];
    let term = "";

    const suppressed = () => {};

    $: searchTerm = new RegExp(term, "i");
    $: searchResults = search(items, searchTerm, fields)
</script>

<div>
    <form class="field" on:submit|preventDefault={suppressed}>
        <div class="control">
            <input bind:value={term} class="input" type="text" placeholder="Search..." />
        </div>
    </form>
</div>