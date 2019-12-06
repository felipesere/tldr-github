import App from "./App.svelte";

const data = [
  {
    title: "felipesere/advisor",
    lastCommit: {
      branch: "master",
      on: "14min ago",
      by: "felipesere",
      sha1: "a11dfa26e15f4",
      comment: "Add new questions"
    },
    activity: {
      master: {
        commits: 8
      },
      prs: [
        {
          link: "#",
          title: "Add new JSON backend for qeustions",
          by: "cgockel"
        },
        {
          link: "#",
          title: "Use FSUnit for testing",
          by: "fsere"
        },
        {
          link: "#",
          title: "Introduce external advisors",
          by: "cfereday"
        }
      ],
      issues: [
        {
          link: "#",
          title: "Crashes using non-ASCII characters",
          by: "ukutaht"
        },
        {
          link: "#",
          title: "Needs more questions about leadership",
          by: "ndyer"
        },
        {
          link: "#",
          title: "Pictures are to slow on mobile",
          by: "mulliestephenson"
        }
      ]
    }
  },
  {
    title: "async-rs/async-std",
    lastCommit: {
      branch: "master",
      on: "2 hours ago",
      by: "yoshwyut",
      sha1: "850b8ae9d06df",
      comment: "Merge pull request #344"
    },
    activity: {
      master: {
        commits: 34
      },
      prs: [
        {
          link: "#",
          title: "Implement DoubleEndedStream",
          by: "felipesere"
        },
        {
          link: "#",
          title: "Make channels faster",
          by: "stjepjang"
        }
      ],
      issues: [
        {
          link: "#",
          title: "Better support for byte ordered reads and writes?",
          by: "yoshwyut"
        },
        {
          link: "#",
          title: "Make errors more verbose",
          by: "zkat"
        }
      ]
    }
  },
  {
    title: "http-rs/tide",
    lastCommit: {
      branch: "master",
      on: "6 days ago",
      by: "stadder",
      sha1: "850b8ae9d06df",
      comment: "Merge pull request #13"
    },
    activity: {
      master: {
        commits: 0
      },
      prs: [
        {
          link: "#",
          title: "Refactor API",
          by: "someone"
        }
      ],
      issues: []
    }
  }
];

const app = new App({
  target: document.getElementById('root'),
  props: { repos: data }
});

export default app;
