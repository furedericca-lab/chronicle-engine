import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import { useState, useEffect } from 'react'
import { getSettings, updateSettings } from '../api'
import { Save } from 'lucide-react'

export function Settings() {
  const queryClient = useQueryClient()
  const { data, isLoading } = useQuery({
    queryKey: ['settings'],
    queryFn: getSettings,
  })

  const [localToml, setLocalToml] = useState('')
  const [saveStatus, setSaveStatus] = useState<{message: string; isError: boolean} | null>(null)

  useEffect(() => {
    // Basic mock mapping of config struct to a readable block for the admin
    if (data?.config) {
      const configStr = JSON.stringify(data.config, null, 2)
      setLocalToml(configStr) 
    }
  }, [data])

  const mutation = useMutation({
    mutationFn: (newToml: string) => updateSettings(newToml),
    onSuccess: (res) => {
      setSaveStatus({ message: res.summary || 'Settings updated successfully.', isError: false })
      queryClient.invalidateQueries({ queryKey: ['settings'] })
    },
    onError: (err: any) => {
      setSaveStatus({ message: err.message, isError: true })
    }
  })

  return (
    <div>
      <h1 className="page-title">Settings</h1>
      <p className="page-description">System configuration and tuning (Requires Backend Restart after save).</p>
      
      <div className="card" style={{ marginTop: '24px', display: 'flex', flexDirection: 'column', height: '60vh' }}>
        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '16px' }}>
          <h3>Editor (JSON to TOML proxy)</h3>
          <button 
            className="btn btn-primary" 
            onClick={() => mutation.mutate(localToml)}
            disabled={mutation.isPending}
          >
            <Save size={16} style={{ marginRight: '8px' }} /> Save Changes
          </button>
        </div>
        
        {saveStatus && (
          <div style={{ padding: '12px', marginBottom: '16px', borderRadius: '4px', backgroundColor: saveStatus.isError ? 'rgba(255,0,0,0.1)' : 'rgba(0,255,0,0.1)', color: saveStatus.isError ? '#ff4d4f' : '#4ade80' }}>
            {saveStatus.message}
          </div>
        )}

        {isLoading ? (
          <p>Loading config...</p>
        ) : (
          <textarea 
            className="input"
            style={{ flex: 1, fontFamily: 'monospace', resize: 'none', padding: '16px' }}
            value={localToml}
            onChange={(e) => setLocalToml(e.target.value)}
            spellCheck={false}
          />
        )}
      </div>
    </div>
  )
}
