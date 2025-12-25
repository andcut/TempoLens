import React from "react";
import type { AnalysisOptions } from "../types";

interface Props {
  options: AnalysisOptions;
  onChange: (next: AnalysisOptions) => void;
  onReset: () => void;
}

export function SettingsPanel({ options, onChange, onReset }: Props) {
  const update = (patch: Partial<AnalysisOptions>) => {
    onChange({ ...options, ...patch });
  };

  return (
    <div className="panel stack settings-panel">
      <div className="panel-title">Settings</div>

      <div className="field-row">
        <label className="field field-small">
          <span>Depth</span>
          <input
            type="number"
            min={8}
            max={24}
            value={options.depth}
            onChange={(e) => update({ depth: parseInt(e.target.value, 10) || 14 })}
          />
        </label>
        <label className="field field-small">
          <span>MultiPV</span>
          <input
            type="number"
            min={1}
            max={6}
            value={options.multipv}
            onChange={(e) => update({ multipv: parseInt(e.target.value, 10) || 4 })}
          />
        </label>
        <label className="field field-small">
          <span>Move time (ms)</span>
          <input
            type="number"
            min={0}
            value={options.movetime_ms ?? ""}
            onChange={(e) =>
              update({ movetime_ms: e.target.value ? parseInt(e.target.value, 10) : null })
            }
          />
        </label>
      </div>

      <div className="field-row">
        <label className="field field-small">
          <span>Threads</span>
          <input
            type="number"
            min={1}
            value={options.threads ?? ""}
            onChange={(e) => update({ threads: e.target.value ? parseInt(e.target.value, 10) : null })}
          />
        </label>
        <label className="field field-small">
          <span>Hash (MB)</span>
          <input
            type="number"
            min={0}
            value={options.hash_mb ?? ""}
            onChange={(e) => update({ hash_mb: e.target.value ? parseInt(e.target.value, 10) : null })}
          />
        </label>
        <label className="field field-small">
          <span>TimeControl</span>
          <input
            value={options.time_control ?? ""}
            onChange={(e) => update({ time_control: e.target.value || null })}
            placeholder="180+2"
          />
        </label>
      </div>

      <div className="field-row">
        <label className="field field-small">
          <span>Alpha</span>
          <input
            type="number"
            step="0.1"
            value={options.alpha}
            onChange={(e) => update({ alpha: parseFloat(e.target.value) || 2.0 })}
          />
        </label>
        <label className="field field-small">
          <span>Beta</span>
          <input
            type="number"
            step="0.1"
            value={options.beta}
            onChange={(e) => update({ beta: parseFloat(e.target.value) || 10.0 })}
          />
        </label>
        <label className="field field-small">
          <span>k (sigmoid)</span>
          <input
            type="number"
            step="0.1"
            value={options.k_sigmoid}
            onChange={(e) => update({ k_sigmoid: parseFloat(e.target.value) || 1.2 })}
          />
        </label>
      </div>

      <div className="field-row">
        <label className="field field-small">
          <span>Pressure pivot</span>
          <input
            type="number"
            step="1"
            value={options.time_pressure_pivot}
            onChange={(e) => update({ time_pressure_pivot: parseFloat(e.target.value) || 30.0 })}
          />
        </label>
        <label className="field field-small">
          <span>Pressure scale</span>
          <input
            type="number"
            step="1"
            value={options.time_pressure_scale}
            onChange={(e) => update({ time_pressure_scale: parseFloat(e.target.value) || 8.0 })}
          />
        </label>
        <label className="field field-small">
          <span>Pressure boost</span>
          <input
            type="number"
            step="0.1"
            value={options.time_pressure_boost}
            onChange={(e) => update({ time_pressure_boost: parseFloat(e.target.value) || 3.0 })}
          />
        </label>
      </div>

      <div className="settings-actions">
        <button className="ghost" onClick={onReset}>Reset defaults</button>
      </div>
    </div>
  );
}
