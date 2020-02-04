<script>
    import {fade, fly} from 'svelte/transition'
</script>

<div class="flex flex-col items-center fixed top-0 left-0 right-0 bottom-0">

    <div transition:fade={{duration: 100}} class="z-20 fixed top-0 left-0 right-0 bottom-0 bg-black opacity-75"></div>

    <div in:fly={{y: 50, duration: 250}} out:fly={{y:-50, duration: 100}} class="z-30 mt-12 fixed-size rounded-lg flex flex-col overflow-scroll bg-white">
        <header class="border-b-2 border-gray-400 shadow bg-gray-200 flex flex-row p-4">
            <slot name="title" />
        </header>
        <section class="bg-white overflow-auto p-5 flex-grow">
            <slot name="body" />
        </section>
        <footer class="border-t-2 border-gray-400 shadow bg-gray-200 flex flex-row p-4">
            <slot name="footer" />
        </footer>
    </div>
</div>

<style>
    .fixed-size {
        width: 900px;
        max-height: calc(100vh - 250px);
    }
</style>
