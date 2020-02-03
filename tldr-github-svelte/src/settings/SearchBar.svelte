<script>
    import {search} from './search'

    export let items;
    export let searchResults = items;
    let fields = ["title", "by", "labels"];
    let term = "";

    $: searchTerm = new RegExp(term, "i");
    $: searchResults = search(items, searchTerm, fields)
</script>

<input bind:value={term}
       class="w-full rounded p-2 bg-gray-200 border-gray-300 hover:border-gray-300 focus:outline-none focus:bg-white focus:shadow-outline focus:border-blue-300"
       type="text" placeholder="Search..."/>
