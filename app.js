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

var root = document.getElementById("app");

var LastCommit = {
  view: vnode =>
    m("div", [
      "Last commit on ",
      m("code", vnode.attrs.branch),
      " ",
      vnode.attrs.on,
      " by ",
      vnode.attrs.by,
      ":",
      m("div.commit", vnode.attrs.comment)
    ])
};

var Master = {
  view: vnode =>
    m("div", [vnode.attrs.commits, " new commits on ", m("code", "master")])
};

var GlowBox = {
  view: vnode => {
    return m("div.glow-box.is-size-7", [
      m("a", { href: vnode.attrs.link }, vnode.attrs.title),
      m("div", `by ${vnode.attrs.by}`)
    ]);
  }
};

var PullRequests = {
  view: vnode => {
    return m("div", [
      vnode.attrs.prs.length,
      " new Pull Requests:",
      vnode.attrs.prs.map(pr => m(GlowBox, pr))
    ]);
  }
};

var Issues = {
  view: vnode => {
    return m("div", [
      vnode.attrs.issues.length,
      " new Issues:",
      vnode.attrs.issues.map(issue => m(GlowBox, issue))
    ]);
  }
};

var Recent = {
  view: vnode => 
    m("div", [
      m("h2.recent-activity", "Recent activity"),
      m(Master, vnode.attrs.master)
    ]),
}

var Activity = {
  view: vnode => [
    m(Recent, {master: vnode.attrs.master}),
    m("div.stack", [
      m(PullRequests, { prs: vnode.attrs.prs }),
      m(Issues, { issues: vnode.attrs.issues })
    ])
  ]
};

var Repo = {
  view: vnode => {
    return m("article.card", [
      m("header.card-header", m("p.card-header-title", vnode.attrs.title)),
      m(
        "div.card-content",
        m("div.content.stack", [
          m(LastCommit, vnode.attrs.lastCommit),
          m(Activity, vnode.attrs.activity)
        ])
      ),
      m('footer.card-footer', m('p.is-size-7.card-footer-item', "Last update 2min ago"))
    ]);
  }
};

var AllRepos = {
  view: vnode => {
    return m("div.grid", vnode.attrs.repos.map(repo => m(Repo, repo)));
  }
};

m.render(root, m(AllRepos, { repos: data }));
