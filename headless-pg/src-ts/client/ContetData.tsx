import React, { useState, useEffect } from 'react';
import { Item, NewItem } from './types/Item';
import { itemsApi } from './ContetData/api';
import dataUtil from './ContetData/dataUtil';
import ItemDialog from './ContetData/ItemDialog';
import Head from '../components/Head';
let contentId: number = 0;

function App() {
  const [items, setItems] = useState<Item[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [dialogOpen, setDialogOpen] = useState(false);
  const [dialogMode, setDialogMode] = useState<'create' | 'edit'>('create');
  const [editingItem, setEditingItem] = useState<undefined>();

  // アイテム一覧を取得
  const fetchItems = async () => {
    try {
      setLoading(true);
      const data = await itemsApi.getAll(contentId);
      //console.log(data);
      setItems(data);
    } catch (err) {
      setError('アイテムの取得に失敗しました');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    const searchParams = new URLSearchParams(window.location.search);
    contentId = searchParams.get('content') || "";
    console.log("contentId=", contentId);
    fetchItems();
  }, []);

  // 新規作成ダイアログを開く
  const handleCreate = () => {
    setDialogMode('create');
    setEditingItem(undefined);
    setDialogOpen(true);
  };

  // 編集ダイアログを開く
  const handleEdit = (item: Item) => {
    setDialogMode('edit');
    setEditingItem(item);
    setDialogOpen(true);
  };

  // アイテム保存
  const handleSave = async (itemData: NewItem) => {
    try {
      console.log(itemData);
      if (dialogMode === 'create') {
        await itemsApi.create(itemData);
      } else if (editingItem) {
        await itemsApi.update(editingItem.id, itemData);
      }
      await fetchItems();
      setError(null);
    } catch (err) {
      setError('保存に失敗しました');
    }
  };

  // アイテム削除
  const handleDelete = async (id: number) => {
    if (!confirm('本当に削除しますか？')) return;
    
    try {
      await itemsApi.delete(id);
      await fetchItems();
      setError(null);
    } catch (err) {
      setError('削除に失敗しました');
    }
  };

  if (loading) {
    return (
      <div className="min-h-screen bg-gray-100 flex items-center justify-center">
        <div className="text-xl">読み込み中...</div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-100 pt-1 pb-8">
      <Head />
      <div className="max-w-6xl mx-auto px-4">
        <a href="/">
          <button
            className="bg-white text-black my-2 px-4 py-2 rounded-md hover:bg-blue-700"
          >
          Back
          </button>
        </a>
        <div className="bg-white rounded-lg shadow">
          <div className="px-6 py-4 border-b border-gray-200 flex justify-between items-center">
            <h1 className="text-2xl font-bold text-gray-900">Data</h1>
          </div>

          <div className="p-6">
            {items.length === 0 ? (
              <div className="text-center py-8 text-gray-500">
                アイテムがありません
              </div>
            ) : (
              <div className="overflow-x-auto">
                <table className="min-w-full divide-y divide-gray-200">
                  <thead className="bg-gray-50">
                    <tr>
                      <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                        ID
                      </th>
                      <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                        data
                      </th>
                      <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                        操作
                      </th>
                    </tr>
                  </thead>
                  <tbody className="bg-white divide-y divide-gray-200">
                    {items.map((item) => (
                      <tr key={item.id}>
                        <td className="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
                          {item.id}
                        </td>
                        <td className="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
                          {JSON.stringify(item.data)}
                        </td>
                        <td className="px-6 py-4 whitespace-nowrap text-sm font-medium">
                          <button
                            onClick={() => handleEdit(item)}
                            className="text-indigo-600 hover:text-indigo-900 mr-4"
                          >
                            [ Show ]
                          </button>       
                          {/*
                          <button
                            onClick={() => handleDelete(item.id)}
                            className="text-red-600 hover:text-red-900"
                          >
                            削除
                          </button>
                          */}                   

                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            )}
          </div>
        </div>
      </div>

      <ItemDialog
        isOpen={dialogOpen}
        onClose={() => setDialogOpen(false)}
        onSave={handleSave}
        item={editingItem}
        mode={dialogMode}
      />
    </div>
  );
}

export default App;