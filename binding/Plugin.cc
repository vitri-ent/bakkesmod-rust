#include <memory>

#include <bakkesmod/wrappers/cvarmanagerwrapper.h>
#include <bakkesmod/wrappers/GameWrapper.h>
#include <bakkesmod/plugin/bakkesmodsdk.h>

extern "C" {
	void bmrs_pluginOnLoad(CVarManagerWrapper *cvm, GameWrapper *game);
	void bmrs_pluginOnUnload();
}

namespace bmrs {
	class Plugin: BakkesMod::Plugin::BakkesModPlugin {
		virtual void onLoad() override {
			bmrs_pluginOnLoad(this->cvarManager.get(), this->gameWrapper.get());
		}

		virtual void onUnload() override {
			bmrs_pluginOnUnload();
		}	
	};
	static std::shared_ptr<bmrs::Plugin> pluginSingleton;
}

extern "C" {
	BAKKESMOD_PLUGIN_EXPORT uintptr_t bmrs_pluginInit() {
		if (!bmrs::pluginSingleton) {
			bmrs::pluginSingleton = std::shared_ptr<bmrs::Plugin>(new bmrs::Plugin());
		}
		return reinterpret_cast<uintptr_t>(&bmrs::pluginSingleton);
	}

	BAKKESMOD_PLUGIN_EXPORT void bmrs_pluginUninit() {
		bmrs::pluginSingleton = nullptr;
	}
}
