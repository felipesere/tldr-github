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
          by: "molliestephenson"
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
          by: "felipesere",
        },
        {
          link: "#",
          title: "Make channels faster",
          by: "stjepjang",
        },
      ],
      issues: [
        {
          link: "#",
          title: "Better support for byte ordered reads and writes?",
          by: "yoshwyut",
        },
        {
          link: "#",
          title: "Make errors more verbose",
          by: "zkat",
        },
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
          by: "someone",
        },
      ],
      issues: []
    }
  }
];

var root = document.getElementById("app");

var LastCommit = {
  view: vnode => [
    "Last commit on ",
    m("em", vnode.attrs.branch),
    " ",
    vnode.attrs.on,
    " by ",
    vnode.attrs.by,
    " - ",
    vnode.attrs.comment
  ]
};

var Master = {
  view: vnode => [vnode.attrs.commits, " new commits on master"]
};

var Link = {
  view: vnode => m("a", { href: vnode.attrs.link }, vnode.attrs.title)
};

var PullRequests = {
  view: vnode => {
    return [
      vnode.attrs.prs.length,
      " new Pull Requests:",
      m("ol", vnode.attrs.prs.map(pr => m("li", [m(Link, pr), " by ", pr.by])))
    ];
  }
};

var Issues = {
  view: vnode => {
    return [
      vnode.attrs.issues.length,
      " new Issues:",
      m(
        "ol",
        vnode.attrs.issues.map(issue =>
          m("li", [m(Link, issue), " by ", issue.by])
        )
      )
    ];
  }
};

var RecentActivity = {
  view: vnode => [
    m("h4", "Recent activity"),
    m(Master, vnode.attrs.master),
    m(PullRequests, { prs: vnode.attrs.prs }),
    m(Issues, { issues: vnode.attrs.issues })
  ]
};

var Repo = {
  view: vnode => {
    return m("article.card", [
      m("p.card-header-title", vnode.attrs.title),
      m("div.card-content", [
        m(LastCommit, vnode.attrs.lastCommit),
        m(RecentActivity, vnode.attrs.activity),
      ])
    ]);
  }
};

var AllRepos = {
  view: vnode => {
    return m("div.grid", vnode.attrs.repos.map(repo => m(Repo, repo)))
  }
}

m.render(root, m(AllRepos, {repos: data}))
