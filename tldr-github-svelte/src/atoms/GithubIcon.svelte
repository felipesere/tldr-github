<script>
    // because jest+babel and svelte can't agree on imports
    import * as octicons from '@primer/octicons'
    let allIcons = octicons.default || octicons;

    export let icon = ""
</script>


<div>
    {#if !icon }
    <!-- nothing -->
    {:else}
        {@html allIcons[icon].toSVG()}
    {/if}
</div>

<style>
    div {
        transform: translateY(2px);
        margin-left: 2px;
        margin-right: 4px;
    }
</style>
