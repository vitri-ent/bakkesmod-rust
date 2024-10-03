#include <string>

#include <bakkesmod/wrappers/cvarmanagerwrapper.h>
#include <bakkesmod/wrappers/cvarwrapper.h>

#include "./Common.hh"
#include "./CVar.hh"
#include "./CVarManager.hh"

extern "C" {
	void bmrsCVarManager_execute_command(const bmrsCVarManager *self, bmrsString *command, bool log) {
		CVarManagerWrapper *native = (CVarManagerWrapper *)self;
		native->executeCommand(bmrs::ConvertString(command), log);
	}

	void bmrsCVarManager_register_notifier(
		const bmrsCVarManager *self,
		bmrsString *cvar,
		bmrsCommandNotifier notifier,
		void *aux,
		bmrsString *description,
		unsigned char permissions
	) {
		CVarManagerWrapper *native = (CVarManagerWrapper *)self;
		native->registerNotifier(
			bmrs::ConvertString(cvar),
			[=](std::vector<std::string> args) {
				std::vector<bmrsString> bmrsArgs(args.size());
				for (size_t i = 0; i < args.size(); i++) {
					bmrsArgs[i] = bmrs::ConvertString(args[i]);
				}
				notifier(bmrsArgs.data(), bmrsArgs.size(), aux);
			},
			bmrs::ConvertString(description),
			permissions
		);
	}

	bool bmrsCVarManager_remove_notifier(const bmrsCVarManager *self, bmrsString *cvar) {
		CVarManagerWrapper *native = (CVarManagerWrapper *)self;
		return native->removeNotifier(bmrs::ConvertString(cvar));		
	}

	bmrsCVar *bmrsCVarManager_register_cvar(
		const bmrsCVarManager *self,
		bmrsString *name,
		bmrsString *defaultValue,
		bmrsString *description,
		bool searchable,
		bool hasMin,
		float min,
		bool hasMax,
		float max,
		bool saveToCfg
	) {
		CVarManagerWrapper *native = (CVarManagerWrapper *)self;
		CVarWrapper cvar = native->registerCvar(
			bmrs::ConvertString(name),
			bmrs::ConvertString(defaultValue),
			bmrs::ConvertString(description),
			searchable,
			hasMin,
			min,
			hasMax,
			max,
			saveToCfg
		);
		return bmrs::ConvertCVar(cvar);
	}

	bool bmrsCVarManager_remove_cvar(const bmrsCVarManager *self, bmrsString *cvar) {
		CVarManagerWrapper *native = (CVarManagerWrapper *)self;
		return native->removeCvar(bmrs::ConvertString(cvar));		
	}

	void bmrsCVarManager_log(const bmrsCVarManager *self, bmrsString *message) {
		CVarManagerWrapper *native = (CVarManagerWrapper *)self;
		native->log(bmrs::ConvertString(message));
	}

	bmrsCVar *bmrsCVarManager_get_cvar(const bmrsCVarManager *self, bmrsString *name) {
		CVarManagerWrapper *native = (CVarManagerWrapper *)self;
		return bmrs::ConvertCVar(native->getCvar(bmrs::ConvertString(name)));
	}

	bmrsString bmrsCVarManager_get_bind_string_for_key(const bmrsCVarManager *self, bmrsString *key) {
		CVarManagerWrapper *native = (CVarManagerWrapper *)self;
		return bmrs::ConvertString(native->getBindStringForKey(bmrs::ConvertString(key)));
	}

	void bmrsCVarManager_set_bind(const bmrsCVarManager *self, bmrsString *key, bmrsString *command) {
		CVarManagerWrapper *native = (CVarManagerWrapper *)self;
		native->setBind(bmrs::ConvertString(key), bmrs::ConvertString(command));
	}
	
	void bmrsCVarManager_remove_bind(const bmrsCVarManager *self, bmrsString *key) {
		CVarManagerWrapper *native = (CVarManagerWrapper *)self;
		native->removeBind(bmrs::ConvertString(key));
	}

	bmrsString bmrsCVarManager_get_alias(const bmrsCVarManager *self, bmrsString *alias) {
		CVarManagerWrapper *native = (CVarManagerWrapper *)self;
		return bmrs::ConvertString(native->getAlias(bmrs::ConvertString(alias)));
	}

	void bmrsCVarManager_set_alias(const bmrsCVarManager *self, bmrsString *key, bmrsString *script) {
		CVarManagerWrapper *native = (CVarManagerWrapper *)self;
		native->setAlias(bmrs::ConvertString(key), bmrs::ConvertString(script));
	}

	void bmrsCVarManager_backup_cfg(const bmrsCVarManager *self, bmrsString *path) {
		CVarManagerWrapper *native = (CVarManagerWrapper *)self;
		native->backupCfg(bmrs::ConvertString(path));
	}

	void bmrsCVarManager_backup_binds(const bmrsCVarManager *self, bmrsString *path) {
		CVarManagerWrapper *native = (CVarManagerWrapper *)self;
		native->backupBinds(bmrs::ConvertString(path));
	}

	void bmrsCVarManager_load_cfg(const bmrsCVarManager *self, bmrsString *path) {
		CVarManagerWrapper *native = (CVarManagerWrapper *)self;
		native->loadCfg(bmrs::ConvertString(path));
	}
};
