import * as React from 'react'
import { GithubIcon, IconName } from './atoms/GithubIcon'
import { Indicator } from './atoms/Indicator'
import { GlowBox } from './atoms/GlowBox'
import { defaultTab, TabNames, Tabs } from './Tabs'
import { useState } from 'react'

type Item = {
    last_updated: string
    link: string
    title: string
    by: string
}

type Activity = {
    issues: Item[]
    prs: Item[]
}

type Repo = {
    title: string
    activity: Activity
}

const anyActivity = (repo: Repo): boolean =>
    repo.activity.issues.length + repo.activity.prs.length > 0

const itemsOf = (
    { activity: { issues, prs } }: Repo,
    tab: TabNames
): Item[] => {
    switch (tab) {
        case 'all':
            return [...issues, ...prs]
        case 'issues':
            return [...issues]
        case 'prs':
            return [...prs]
    }
}

type RepoProps = {
    repo: Repo
}

export function Repo({ repo }: RepoProps): JSX.Element {
    const [currentTab, setCurrentTab] = useState<TabNames>(defaultTab)

    return (
        <article className="border border-gray-300 shadow-md max-w-full flex flex-col">
            <header className="shadow bg-gray-200 border-gray-400 border-b-2">
                <div className="py-3 px-6 flex flex-grow font-bold">
                    <p className="flex-grow text-gray-700 leading-loose">
                        {repo.title}
                    </p>
                    <a
                        className="text-gray-600 fill-current"
                        data-testid="settings"
                        href="#"
                    >
                        <GithubIcon icon="gear" />
                    </a>
                </div>
            </header>

            <div className="p-4">
                <div className="stack">
                    <Tabs className="mb-4" onChangeTab={setCurrentTab} />
                </div>
                {anyActivity(repo) ? (
                    <TrackedItems items={itemsOf(repo, currentTab)} />
                ) : (
                    <p className="text-center text-gray-600">
                        No items are being tracked...
                    </p>
                )}
            </div>
        </article>
    )
}

type TrackedItemsProps = {
    items: Item[]
}

function TrackedItems({ items }: TrackedItemsProps): JSX.Element {
    return (
        <ul className="stack-sm ml-0 list-none">
            {items.map(item => (
                <li className="flex">
                    <Indicator time={item.last_updated} />
                    <GlowBox content={item} />
                </li>
            ))}
        </ul>
    )
}
