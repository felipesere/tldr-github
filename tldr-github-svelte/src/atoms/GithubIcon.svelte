<script>
    import octicons from '@primer/octicons'

    export let icon = ""
</script>


<div>
    {#if !icon }
    <!-- nothing -->
    {:else}
        {@html octicons[icon].toSVG()}
    {/if}
</div>

<style>
    div {
        transform: translateY(2px);
        margin-left: 2px;
        margin-right: 4px;
    }
</style>
