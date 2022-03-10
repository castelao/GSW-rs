% Matlab script to extract checking values from reference dataset
 

load gsw_data_v3_0.mat

fields = fieldnames(gsw_cv)

% varnames = {'f', 'grav', 'distance', 'lat_chck_cast', 'p_chck_cast', 'SA_chck_cast', 'CT_chck_cast', 't_chck_cast', 'SP_from_C', 'C_from_SP', 'SP_from_R', 'R_from_SP', 'SP_from_SK', 'SA_from_SP', 'Sstar_from_SP', 'CT_from_t', 'deltaSA_from_SP', 'SR_from_SP', 'SP_from_SA', 'pt_from_CT', 't_from_CT', 't90_from_t48', 't90_from_t68', 'z_from_p' 'p_from_z' 'z_from_depth', 'specvol', 'alpha', 'beta', 'specvol_anom', 'specvol_anom_standard', 'rho', 'sigma0', 'sigma1', 'sigma2', 'sigma3', 'sigma4', 'enthalpy', 'enthalpy_diff', 'dynamic_enthalpy', 'sound_speed', 'internal_energy', 'SA_from_rho', 'CT_from_rho', 'CT_maxdensity', 'specvol_t_exact'};

% for v=varnames
%     % specvol = gsw_cv.specvol
%     eval([char(v), ' = gsw_cv.', char(v), ';'])
% end


for i=1:size(fields)
    v = char(fields(i))
    eval([char(v), ' = gsw_cv.', char(v), ';']);
end

command = 'save(''gsw_data_v3_0.chck_cast.mat''';
% command = [command, ', ''', strjoin(varnames, ''', '''), ''', ''-v6'');']
command = [command, ', ''version_number'', ''version_date'', ''', strjoin(fields, ''', '''), ''', ''-v6'');']
eval(command)
