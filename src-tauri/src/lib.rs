use std::fs;
use std::path::PathBuf;
use tauri::Manager;

/// מחזיר את הנתיב לקובץ הנתונים הקבוע של האפליקציה,
/// בתיקיית הנתונים הסטנדרטית של המשתמש (לא תלוי ב-localStorage כלל).
fn get_data_file_path(app_handle: &tauri::AppHandle) -> Result<PathBuf, String> {
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("שגיאה באיתור תיקיית הנתונים: {}", e))?;

    if !app_dir.exists() {
        fs::create_dir_all(&app_dir).map_err(|e| format!("שגיאה ביצירת תיקיית הנתונים: {}", e))?;
    }

    Ok(app_dir.join("trading-journal-data.json"))
}

/// טוען את נתוני היומן מהקובץ הקבוע בדיסק.
/// אם הקובץ לא קיים עדיין (הפעלה ראשונה), מחזיר מחרוזת ריקה -
/// ה-JavaScript בצד הלקוח יודע להשתמש בנתוני ברירת מחדל במקרה כזה.
#[tauri::command]
fn load_journal_data(app_handle: tauri::AppHandle) -> Result<String, String> {
    let path = get_data_file_path(&app_handle)?;
    if !path.exists() {
        return Ok(String::new());
    }
    fs::read_to_string(&path).map_err(|e| format!("שגיאה בקריאת קובץ הנתונים: {}", e))
}

/// שומר את נתוני היומן לקובץ הקבוע בדיסק - כתיבה ישירה, אמינה,
/// ולא תלויה באף מנגנון אחסון של הדפדפן.
#[tauri::command]
fn save_journal_data(app_handle: tauri::AppHandle, data: String) -> Result<(), String> {
    let path = get_data_file_path(&app_handle)?;
    fs::write(&path, data).map_err(|e| format!("שגיאה בשמירת קובץ הנתונים: {}", e))
}

/// מחזיר את הנתיב המלא לקובץ הנתונים, כדי שניתן יהיה להציג אותו למשתמש
/// (שקיפות - המשתמש יודע בדיוק איפה הקובץ שלו נמצא).
#[tauri::command]
fn get_data_file_location(app_handle: tauri::AppHandle) -> Result<String, String> {
    let path = get_data_file_path(&app_handle)?;
    Ok(path.to_string_lossy().to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            load_journal_data,
            save_journal_data,
            get_data_file_location
        ])
        .run(tauri::generate_context!())
        .expect("שגיאה בהפעלת האפליקציה");
}
