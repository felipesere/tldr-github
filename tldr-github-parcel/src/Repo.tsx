import * as React from "react";
import { GithubIcon, IconName } from "./atoms/GithubIcon";
import { Indicator } from "./atoms/Indicator";
import { GlowBox } from "./atoms/GlowBox";

type Item = {
  last_updated: string;
  link: string;
  title: string;
  by: string;
};

type Activity = {
  issues: Item[];
  prs: Item[];
};

type Repo = {
  title: string;
  activity: Activity;
};

const anyActivity = (repo: Repo): boolean =>
  repo.activity.issues.length + repo.activity.prs.length > 0;

const itemsOf = ({ activity }: Repo): Item[] => [
  ...activity.issues,
  ...activity.prs
];

type Tab = {
  text: string;
  icon: IconName;
};
const tabs: Tab[] = [];

type RepoProps = {
  repo: Repo;
};

export function Repo({ repo }: RepoProps): JSX.Element {
  return (
    <article className="border border-gray-300 shadow-md max-w-full flex flex-col">
      <header className="shadow bg-gray-200 border-gray-400 border-b-2">
        <div className="py-3 px-6 flex flex-grow font-bold">
          <p className="flex-grow text-gray-700 leading-loose">{repo.title}</p>
          <a
            className="text-gray-600 fill-current"
            data-testid="settings"
            href="#"
          >
            <GithubIcon icon="gear" />
          </a>
        </div>
      </header>

      <div className="stack">
        <ul className="flex border-b list-none">
          {tabs.map(tab => {
            return (
              <li>
                <a className="cursor-pointer">
                  <GithubIcon icon={tab.icon} />
                  <span>{tab.text}</span>
                </a>
              </li>
            );
          })}
        </ul>
      </div>
      {anyActivity(repo) ? (
        <TrackedItems items={itemsOf(repo)} />
      ) : (
        <p className="text-center text-gray-600">
          No items are being tracked...
        </p>
      )}
    </article>
  );
}

type TrackedItemsProps = {
  items: Item[];
};

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
  );
}
