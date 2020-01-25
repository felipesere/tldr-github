<script>
    export let onClickAdd
</script>

<div class="side-nav stack">
    <button type="button" class="nav-button" on:click={onClickAdd}>
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20">
            <path d="M16 10c0 .553-.048 1-.601 1H11v4.399c0 .552-.447.601-1 .601-.553 0-1-.049-1-.601V11H4.601C4.049 11 4 10.553 4 10c0-.553.049-1 .601-1H9V4.601C9 4.048 9.447 4 10 4c.553 0 1 .048 1 .601V9h4.399c.553 0 .601.447.601 1z"/>
        </svg>
    </button>
    <button type="button" class="nav-button">
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16">
            <path fill-rule="evenodd" d="M16 14v1H0V0h1v14h15zM5 13H3V8h2v5zm4 0H7V3h2v10zm4 0h-2V6h2v7z"></path>
        </svg>
    </button>
    <button type="button" class="nav-button">
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 14 16">
            <path d="M8 13H6V6h5v2H8v5zM7 1C4.81 1 2.87 2.02 1.59 3.59L0 2v4h4L2.5 4.5C3.55 3.17 5.17 2.3 7 2.3c3.14 0 5.7 2.56 5.7 5.7s-2.56 5.7-5.7 5.7A5.71 5.71 0 011.3 8c0-.34.03-.67.09-1H.08C.03 7.33 0 7.66 0 8c0 3.86 3.14 7 7 7s7-3.14 7-7-3.14-7-7-7z"></path>
        </svg>
    </button>
</div>

<style>
    .side-nav {
        flex: none;
        background-color: #2C5282;
        width: 4rem;
        padding-top: 4rem;
    }

    .nav-button {
        align-items: center;
        background: white;
        border-radius: .5rem;
        color: black;
        display: flex;
        height: 2.5rem;
        justify-content: center;
        margin-left: auto;
        margin-right: auto;
        margin-bottom: 1rem;;
        opacity: 0.5;
        overflow: hidden;
        padding: 5px;
        width: 2.5rem;
    }

    .nav-button:hover {
        box-shadow: 0 0 5px black;
        opacity: 1;
    }

    .nav-button svg {
        height: 1.5rem;
        width: 1.5rem;
        display: block;
        fill: currentColor;
    }
</style>