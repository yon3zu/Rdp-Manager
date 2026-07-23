import { useMemo, useState } from "react";
import { useStore } from "../../state/store";
import type { ConnectionProfile, Group } from "../../api/types";

interface TreeNode {
  group: Group | null; // null = root / ungrouped bucket
  children: TreeNode[];
  profiles: ConnectionProfile[];
}

function buildTree(groups: Group[], profiles: ConnectionProfile[]): TreeNode {
  const byParent = new Map<string | null, Group[]>();
  for (const g of groups) {
    const key = g.parent_id;
    if (!byParent.has(key)) byParent.set(key, []);
    byParent.get(key)!.push(g);
  }
  const profilesByGroup = new Map<string | null, ConnectionProfile[]>();
  for (const p of profiles) {
    const key = p.group_id;
    if (!profilesByGroup.has(key)) profilesByGroup.set(key, []);
    profilesByGroup.get(key)!.push(p);
  }

  function build(groupId: string | null, group: Group | null): TreeNode {
    const childGroups = byParent.get(groupId) ?? [];
    return {
      group,
      children: childGroups.map((g) => build(g.id, g)),
      profiles: profilesByGroup.get(groupId) ?? [],
    };
  }

  return build(null, null);
}

function GroupNode({ node, depth }: { node: TreeNode; depth: number }) {
  const [expanded, setExpanded] = useState(true);
  const {
    selectedProfileId,
    selectProfile,
    startNewProfile,
    renameGroup,
    deleteGroup,
    createGroup,
    launchConnection,
    duplicateProfile,
    deleteProfile,
    activeSessionIds,
  } = useStore();
  const [renaming, setRenaming] = useState(false);
  const [renameValue, setRenameValue] = useState(node.group?.name ?? "");

  const hasChildren = node.children.length > 0 || node.profiles.length > 0;

  return (
    <div>
      {node.group && (
        <div
          className="group flex items-center gap-1 px-2 py-1 rounded hover:bg-neutral-100 dark:hover:bg-neutral-800 text-sm"
          style={{ paddingLeft: depth * 14 + 8 }}
        >
          <button
            onClick={() => setExpanded((e) => !e)}
            className="w-4 text-neutral-400 shrink-0"
          >
            {hasChildren ? (expanded ? "▾" : "▸") : ""}
          </button>
          {renaming ? (
            <input
              autoFocus
              value={renameValue}
              onChange={(e) => setRenameValue(e.target.value)}
              onBlur={() => {
                setRenaming(false);
                if (renameValue.trim() && renameValue !== node.group!.name) {
                  renameGroup(node.group!.id, renameValue.trim());
                }
              }}
              onKeyDown={(e) => {
                if (e.key === "Enter") (e.target as HTMLInputElement).blur();
              }}
              className="flex-1 bg-white dark:bg-neutral-700 border border-blue-500 rounded px-1 text-sm"
            />
          ) : (
            <span
              className="flex-1 truncate cursor-default font-medium text-neutral-700 dark:text-neutral-300"
              onDoubleClick={() => setRenaming(true)}
              title={node.group.name}
            >
              📁 {node.group.name}
            </span>
          )}
          <div className="hidden group-hover:flex gap-1 shrink-0">
            <button
              title="New connection"
              onClick={() => startNewProfile(node.group!.id)}
              className="text-neutral-400 hover:text-blue-600 text-xs px-1"
            >
              +conn
            </button>
            <button
              title="New subgroup"
              onClick={() => {
                const name = prompt("Subgroup name?");
                if (name?.trim()) createGroup(name.trim(), node.group!.id);
              }}
              className="text-neutral-400 hover:text-blue-600 text-xs px-1"
            >
              +grp
            </button>
            <button
              title="Delete group"
              onClick={() => {
                if (confirm(`Delete group "${node.group!.name}"? Connections inside become ungrouped.`)) {
                  deleteGroup(node.group!.id);
                }
              }}
              className="text-neutral-400 hover:text-red-600 text-xs px-1"
            >
              ✕
            </button>
          </div>
        </div>
      )}
      {(expanded || !node.group) && (
        <div>
          {node.children.map((child) => (
            <GroupNode key={child.group!.id} node={child} depth={node.group ? depth + 1 : depth} />
          ))}
          {node.profiles.map((p) => (
            <div
              key={p.id}
              className={`group flex items-center gap-1 px-2 py-1 rounded text-sm cursor-pointer ${
                selectedProfileId === p.id
                  ? "bg-blue-100 dark:bg-blue-900/40"
                  : "hover:bg-neutral-100 dark:hover:bg-neutral-800"
              }`}
              style={{ paddingLeft: (node.group ? depth + 1 : depth) * 14 + 8 }}
              onClick={() => selectProfile(p.id)}
              onDoubleClick={() => launchConnection(p.id)}
            >
              <span className="w-4 shrink-0" />
              <span
                className={`w-1.5 h-1.5 rounded-full shrink-0 ${
                  activeSessionIds.has(p.id) ? "bg-green-500" : "bg-transparent"
                }`}
                title={activeSessionIds.has(p.id) ? "Connected" : undefined}
              />
              <span className="flex-1 truncate" title={`${p.host}:${p.port}`}>
                🖥️ {p.name}
              </span>
              <div className="hidden group-hover:flex gap-1 shrink-0">
                <button
                  title="Launch"
                  onClick={(e) => {
                    e.stopPropagation();
                    launchConnection(p.id);
                  }}
                  className="text-neutral-400 hover:text-green-600 text-xs px-1"
                >
                  ▶
                </button>
                <button
                  title="Duplicate"
                  onClick={(e) => {
                    e.stopPropagation();
                    duplicateProfile(p.id);
                  }}
                  className="text-neutral-400 hover:text-blue-600 text-xs px-1"
                >
                  ⧉
                </button>
                <button
                  title="Delete"
                  onClick={(e) => {
                    e.stopPropagation();
                    if (confirm(`Delete connection "${p.name}"?`)) deleteProfile(p.id);
                  }}
                  className="text-neutral-400 hover:text-red-600 text-xs px-1"
                >
                  ✕
                </button>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}

export function GroupTree() {
  const { groups, profiles, searchQuery, createGroup, startNewProfile } = useStore();

  const filteredProfiles = useMemo(() => {
    if (!searchQuery.trim()) return profiles;
    const q = searchQuery.toLowerCase();
    return profiles.filter(
      (p) => p.name.toLowerCase().includes(q) || p.host.toLowerCase().includes(q)
    );
  }, [profiles, searchQuery]);

  const tree = useMemo(
    () => buildTree(groups, filteredProfiles),
    [groups, filteredProfiles]
  );

  return (
    <div className="flex flex-col h-full">
      <div className="flex items-center gap-2 p-2 border-b border-neutral-200 dark:border-neutral-700">
        <button
          onClick={() => {
            const name = prompt("Group name?");
            if (name?.trim()) createGroup(name.trim(), null);
          }}
          className="flex-1 text-xs px-2 py-1.5 rounded bg-neutral-200 dark:bg-neutral-700 hover:bg-neutral-300 dark:hover:bg-neutral-600 text-neutral-800 dark:text-neutral-100"
        >
          + Group
        </button>
        <button
          onClick={() => startNewProfile(null)}
          className="flex-1 text-xs px-2 py-1.5 rounded bg-blue-600 hover:bg-blue-500 text-white"
        >
          + Connection
        </button>
      </div>
      <div className="flex-1 overflow-y-auto py-1">
        <GroupNode node={tree} depth={0} />
        {groups.length === 0 && profiles.length === 0 && (
          <p className="text-xs text-neutral-500 px-3 py-4">
            Belum ada koneksi. Klik "+ Connection" untuk mulai.
          </p>
        )}
      </div>
    </div>
  );
}
