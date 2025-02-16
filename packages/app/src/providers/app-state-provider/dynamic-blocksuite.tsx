import { useEffect } from 'react';
import type { Page } from '@blocksuite/store';
import '@blocksuite/blocks';
import { EditorContainer } from '@blocksuite/editor';
import type { LoadWorkspaceHandler, CreateEditorHandler } from './context';
import { getDataCenter } from '@affine/datacenter';

interface Props {
  setLoadWorkspaceHandler: (handler: LoadWorkspaceHandler) => void;
  setCreateEditorHandler: (handler: CreateEditorHandler) => void;
}

const DynamicBlocksuite = ({
  setLoadWorkspaceHandler,
  setCreateEditorHandler,
}: Props) => {
  useEffect(() => {
    const openWorkspace: LoadWorkspaceHandler = async (workspaceId: string) => {
      if (workspaceId) {
        const dc = await getDataCenter();
        return dc.load(workspaceId, { providerId: 'selfhosted' });
      } else {
        return null;
      }
    };

    setLoadWorkspaceHandler(openWorkspace);
  }, [setLoadWorkspaceHandler]);

  useEffect(() => {
    const createEditorHandler: CreateEditorHandler = (page: Page) => {
      const editor = new EditorContainer();
      editor.page = page;
      return editor;
    };

    setCreateEditorHandler(createEditorHandler);
  }, [setCreateEditorHandler]);

  return <></>;
};

export default DynamicBlocksuite;
