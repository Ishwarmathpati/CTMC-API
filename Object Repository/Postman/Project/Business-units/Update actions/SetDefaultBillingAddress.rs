<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>SetDefaultBillingAddress</name>
   <tag></tag>
   <elementGuidId>f22b5ec2-5c3f-4318-b2b9-b52043c38fcd</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;version\&quot;: ${business-unit-version},\n    \&quot;actions\&quot;: [\n        {\n            \&quot;action\&quot; : \&quot;setDefaultBillingAddress\&quot;,\n            \&quot;addressId\&quot; : \&quot;${addressId}\&quot;\n          }\n    ]\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer {{ctp_access_token}}</value>
      <webElementGuid>0942f022-56e7-4ab1-8876-082da4084b07</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>e3ac274b-b093-4188-b113-bc67cb59a081</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${host}/${project-key}/business-units/${business-unit-id}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.host</defaultValue>
      <description></description>
      <id>efa0257d-51c4-4bc7-980c-1b7acc385cf8</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>6e72efaa-77e7-4fea-a7cc-6b132b142fab</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.business-unit-id</defaultValue>
      <description></description>
      <id>0487a707-a71f-4ff2-aa25-8da5738bdd8d</id>
      <masked>false</masked>
      <name>business-unit-id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.business-unit-version</defaultValue>
      <description></description>
      <id>b686277a-bf3c-42e0-a397-4690830187e3</id>
      <masked>false</masked>
      <name>business-unit-version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.addressId</defaultValue>
      <description></description>
      <id>c9e6cc25-ac8a-4c54-afa7-ac92a16fe1de</id>
      <masked>false</masked>
      <name>addressId</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
